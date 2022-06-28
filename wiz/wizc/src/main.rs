use crate::high_level_ir::node_id::TypedModuleId;
use crate::high_level_ir::type_checker::TypeChecker;
use crate::high_level_ir::type_resolver::arena::ResolverArena;
use crate::high_level_ir::type_resolver::TypeResolver;
use crate::high_level_ir::wlib::WLib;
use crate::high_level_ir::{ast2hlir, AstLowering};
use crate::llvm_ir::codegen::CodeGen;
use crate::middle_level_ir::{hlir2mlir, HLIR2MLIR};
use inkwell::context::Context;
use std::error::Error;
use std::io::{Read, Write};
use std::iter::FromIterator;
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};
use wiz_session::Session;
use wiz_syntax::syntax::file::SourceSet;
use wiz_syntax_parser::parser;
use wiz_syntax_parser::parser::wiz::{parse_from_file_path, read_package_from_path};
use wizc_cli::{BuildType, Config};

mod high_level_ir;
mod llvm_ir;
mod middle_level_ir;
mod utils;

fn get_builtin_find_path() -> PathBuf {
    PathBuf::from_iter([env!("HOME"), ".wiz", "lib", "src"])
}

fn get_find_paths() -> Vec<PathBuf> {
    vec![get_builtin_find_path()]
}

fn get_builtin_lib() -> &'static [&'static str] {
    &["core", "std"]
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("{:?}", env::args());
    let app = wizc_cli::app("wizc");
    let matches = app.get_matches();
    let config = Config::from(&matches);

    let mut session = Session::new();
    session.timer("compile", |s| run_compiler(s, config))
}

fn run_compiler(session: &mut Session, config: Config) -> Result<(), Box<dyn Error>> {
    let output = config.output();
    let out_dir = config.out_dir();
    let paths = config.paths();
    let input = config.input();
    let out_dir = out_dir
        .map(PathBuf::from)
        .unwrap_or_else(|| env::current_dir().unwrap());
    let build_type = config.type_().unwrap_or(BuildType::Binary);

    let mlir_out_dir = out_dir.join("mlir");

    let input_source = session.timer::<Result<_, Box<dyn Error>>, _>("parse files", |_| {
        let input_source = if input.is_dir() {
            read_package_from_path(input, config.name())?
        } else {
            SourceSet::File(parse_from_file_path(input)?)
        };
        Ok(input_source)
    })?;

    let mut arena = ResolverArena::default();

    let std_hlir = session.timer("load dependencies", |session| {
        let libraries = config.libraries();

        let std_hlir: parser::result::Result<Vec<_>> = if libraries.is_empty() {
            let find_paths: Vec<_> = get_find_paths().into_iter().chain(paths).collect();

            let mut lib_paths = vec![];

            for lib_name in get_builtin_lib() {
                for p in find_paths.iter() {
                    let lib_path = p.join(lib_name);
                    let package_manifest_path = lib_path.join("Package.wiz");
                    if package_manifest_path.exists() {
                        println!("`{}` found at {}", lib_name, lib_path.display());
                        lib_paths.push(lib_path);
                        break;
                    } else {
                        println!("`{}` Not found at {}", lib_name, lib_path.display());
                    }
                }
            }

            let source_sets = lib_paths
                .iter()
                .map(|p| read_package_from_path(p, None))
                .collect::<parser::result::Result<Vec<_>>>()?;
            Ok(source_sets
                .into_iter()
                .enumerate()
                .map(|(i, s)| ast2hlir(session, &mut arena, s, TypedModuleId::new(i)))
                .collect())
        } else {
            Ok(libraries
                .iter()
                .map(|p| {
                    let lib = WLib::read_from(p);
                    lib.apply_to(&mut arena).unwrap();
                    lib.typed_ir
                })
                .collect())
        };
        std_hlir
    })?;

    let hlfiles = session.timer("resolve type", |session| {
        let mut ast2hlir = AstLowering::new(session, &mut arena);
        ast2hlir.lowing(input_source, TypedModuleId::new(std_hlir.len()))
    })?;

    session.timer("type check", |session| {
        let mut type_checker = TypeChecker::new(session, &arena);
        type_checker.verify(&hlfiles);
    });
    match build_type {
        BuildType::Library => {
            let wlib = WLib::new(hlfiles);
            let wlib_path = out_dir.join(format!("{}.wlib", config.name().unwrap_or_default()));
            wlib.write_to(&wlib_path);
            println!("library written to {}", wlib_path.display());
            return Ok(());
        }
        _ => {}
    };

    println!("===== convert to mlir =====");

    let mut h2m = HLIR2MLIR::new(&arena);

    let std_mlir = std_hlir
        .into_iter()
        .map(|w| h2m.convert_from_source_set(w))
        .collect::<Vec<_>>();

    fs::create_dir_all(&mlir_out_dir)?;
    for m in std_mlir.iter() {
        session.timer(&format!("write mlir `{}`", m.name), |_| {
            let mut f = fs::File::create(mlir_out_dir.join(&m.name))?;
            write!(f, "{}", m.to_string())
        })?;
    }

    let mlfile = hlir2mlir(hlfiles, &std_mlir, &arena)?;

    session.timer(&format!("write mlir `{}`", mlfile.name), |_| {
        let mut f = fs::File::create(mlir_out_dir.join(&mlfile.name))?;
        write!(f, "{}", mlfile.to_string())
    })?;

    println!("==== codegen ====");
    let module_name = &mlfile.name;
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, module_name, config.target_triple());

    for m in std_mlir.into_iter() {
        codegen.file(m);
    }

    codegen.file(mlfile.clone());

    if let Some(emit) = config.emit() {
        let output = if let Some(output) = output {
            PathBuf::from(output)
        } else {
            let mut output_path = PathBuf::from(&mlfile.name);
            output_path.set_extension("ll");
            output_path
        };

        let out_path = out_dir.join(output);

        println!("Output Path -> {}", out_path.display());

        match emit {
            "llvm-ir" => codegen.print_to_file(&out_path),
            "asm" => codegen.write_as_assembly(&out_path),
            _ => codegen.write_as_object(&out_path),
        }?;
    } else {
        let output = if let Some(output) = output {
            String::from(output)
        } else {
            String::from(&mlfile.name)
        };
        let mut ir_file = out_dir.join(&output);
        ir_file.set_extension("ll");
        codegen.print_to_file(&ir_file)?;
        Command::new("clang")
            .args(&[ir_file.to_str().unwrap_or_default(), "-o", &output])
            .exec();
    };
    Ok(())
}
