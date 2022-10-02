use crate::high_level_ir::node_id::TypedModuleId;
use crate::high_level_ir::type_checker::TypeChecker;
use crate::high_level_ir::wlib::WLib;
use crate::high_level_ir::{ast2hlir, AstLowering};
use crate::llvm_ir::codegen::CodeGen;
use crate::middle_level_ir::hlir2mlir;
use inkwell::context::Context;
use std::io::Write;
use std::iter::FromIterator;
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};
use wiz_arena::Arena;
use wiz_result::Result;
use wiz_session::Session;
use wiz_syntax_parser::parser;
use wiz_syntax_parser::parser::wiz::read_package_from_path;
use wizc_cli::{BuildType, Config, ConfigExt};

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
    &["core", "libc", "std"]
}

fn main() -> Result<()> {
    println!("{:?}", env::args());
    let app = wizc_cli::app("wizc");
    let matches = app.get_matches();
    let config = Config::from(&matches);

    let mut session = Session::new(config);
    session.timer("compile", |s| run_compiler(s))
}

fn run_compiler(session: &mut Session) -> Result<()> {
    let config = session.config.clone();
    let output = config.output();
    let out_dir = config.out_dir();
    let paths = config.paths();
    let input = config.input();
    let out_dir = out_dir
        .map(PathBuf::from)
        .unwrap_or_else(|| env::current_dir().unwrap());

    let mlir_out_dir = out_dir.join("mlir");

    let input_source = session.timer::<Result<_>, _>("parse files", |session| {
        Ok(read_package_from_path(
            &session.parse_session,
            input,
            config.name().as_deref(),
        )?)
    })?;

    let mut arena = Arena::default();

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
                        lib_paths.push((lib_path.join("src"), lib_name));
                        break;
                    }
                }
            }

            let source_sets = lib_paths
                .iter()
                .map(|(p, name)| read_package_from_path(&session.parse_session, p, Some(**name)))
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
    match config.type_() {
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

    let std_mlir = std_hlir
        .into_iter()
        .map(|w| hlir2mlir(w, &[], &arena, session, false))
        .collect::<Result<Vec<_>>>()?;

    fs::create_dir_all(&mlir_out_dir)?;
    for m in std_mlir.iter() {
        session.timer(&format!("write mlir `{}`", m.name), |_| {
            let mut f = fs::File::create(mlir_out_dir.join(&m.name))?;
            write!(f, "{}", m.to_string())
        })?;
    }

    let mlfile = hlir2mlir(hlfiles, &std_mlir, &arena, session, true)?;

    session.timer(&format!("write mlir `{}`", mlfile.name), |_| {
        let mut f = fs::File::create(mlir_out_dir.join(&mlfile.name))?;
        write!(f, "{}", mlfile.to_string())
    })?;

    println!("==== codegen ====");
    let module_name = &mlfile.name;
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, module_name, config.target_triple().as_deref());

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

        match emit.as_str() {
            "llvm-ir" => codegen.print_to_file(&out_path),
            "asm" => codegen.write_as_assembly(&out_path),
            _ => codegen.write_as_object(&out_path),
        }?;
    } else {
        let output = output.unwrap_or_else(|| String::from(&mlfile.name));
        let mut ir_file = out_dir.join(&output);
        ir_file.set_extension("ll");
        codegen.print_to_file(&ir_file)?;

        let output = out_dir
            .join(&output)
            .as_os_str()
            .to_string_lossy()
            .to_string();
        Command::new("clang")
            .args(&[ir_file.to_str().unwrap_or_default(), "-o", &output])
            .exec();
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::run_compiler;
    use std::path::PathBuf;
    use wiz_session::Session;
    use wizc_cli::{Config, ConfigBuilder};

    #[test]
    fn compile_file() {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let test_resource_dir = manifest_dir.join("resources/").join("test");
        let repository_root = manifest_dir.join("..").join("..").canonicalize().unwrap();

        let target_file_path = test_resource_dir.join("helloworld.wiz");
        let lib_path = repository_root.join("libraries");
        let out_dir = repository_root.join("out");

        let config = Config::default()
            .input(target_file_path.to_str().unwrap())
            .path(lib_path.to_str().unwrap())
            .out_dir(out_dir.to_str().unwrap());
        let mut session = Session::new(config);
        run_compiler(&mut session).unwrap()
    }
}
