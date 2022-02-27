use crate::high_level_ir::type_resolver::result::Result;
use crate::high_level_ir::type_resolver::TypeResolver;
use crate::high_level_ir::wlib::WLib;
use crate::high_level_ir::{ast2hlir, Ast2HLIR};
use crate::llvm_ir::codegen::CodeGen;
use crate::middle_level_ir::{hlir2mlir, HLIR2MLIR};
use inkwell::context::Context;
use std::error::Error;
use std::io::Write;
use std::option::Option::Some;
use std::path::{Path, PathBuf};
use std::{env, fs, result};
use wiz_session::Session;
use wiz_syntax::parser;
use wiz_syntax::parser::wiz::{parse_from_file_path, read_package_from_path};
use wiz_syntax::syntax::file::SourceSet;
use wizc_cli::{BuildType, Config};

mod constants;
mod ext;
mod high_level_ir;
mod llvm_ir;
mod middle_level_ir;
mod utils;

fn get_builtin_find_path() -> PathBuf {
    let mut std_path = PathBuf::from(env!("HOME"));
    std_path.extend(&[".wiz", "lib", "src"]);
    std_path
}

fn get_find_paths() -> Vec<PathBuf> {
    vec![get_builtin_find_path()]
}

fn get_builtin_lib() -> Vec<&'static str> {
    vec!["core", "std"]
}

fn main() -> result::Result<(), Box<dyn Error>> {
    println!("Args {:?}", env::args());
    let app = wizc_cli::app("wizc");
    let matches = app.get_matches();
    let config = Config::from(&matches);

    let mut session = Session::new();
    session.timer("compile", || run_compiler(config))
}

fn run_compiler(config: Config) -> result::Result<(), Box<dyn Error>> {
    let output = config.output();
    let out_dir = config.out_dir();
    let paths = config.paths();
    let input = config.input();
    let out_dir = out_dir
        .map(PathBuf::from)
        .unwrap_or_else(|| env::current_dir().unwrap());
    let build_type = config.type_().unwrap_or_else(|| BuildType::Binary);

    let mut mlir_out_dir = out_dir.join("mlir");

    println!("=== parse files ===");

    let input_source = if input.is_dir() {
        read_package_from_path(input, config.name())?
    } else {
        SourceSet::File(parse_from_file_path(input)?)
    };

    let mut lib_paths = vec![];

    for l in get_builtin_lib() {
        for mut p in get_find_paths()
            .into_iter()
            .chain(paths.iter().map(PathBuf::from))
        {
            p.extend([l, "Package.wiz"]);
            if p.exists() {
                p.pop();
                println!("`{}` found at {}", l, p.display());
                lib_paths.push(p);
                break;
            } else {
                p.pop();
                println!("`{}` Not found at {}", l, p.display());
            }
        }
    }

    println!("=== load dependencies ===");
    let libraries = config.libraries();

    let std_hlir: Vec<_> = if libraries.is_empty() {
        let source_sets = lib_paths
            .into_iter()
            .map(|p| read_package_from_path(p.as_path(), None))
            .collect::<parser::result::Result<Vec<_>>>()?;
        source_sets.into_iter().map(|s| ast2hlir(s)).collect()
    } else {
        config
            .libraries()
            .iter()
            .map(|p| WLib::read_from(p).typed_ir)
            .collect()
    };

    println!("=== convert to hlir ===");

    let mut ast2hlir = Ast2HLIR::new();

    let hlfiles = ast2hlir.source_set(input_source);

    println!("=== resolve type ===");

    let mut type_resolver = TypeResolver::new();
    type_resolver.global_use(&["core", "builtin", "*"]);
    type_resolver.global_use(&["std", "builtin", "*"]);

    println!("===== detect types =====");
    // detect types
    for s in std_hlir.iter() {
        type_resolver.detect_type_from_source_set(s)?;
    }

    type_resolver.detect_type_from_source_set(&hlfiles)?;

    println!("===== preload decls =====");
    // preload decls
    for hlir in std_hlir.iter() {
        type_resolver.preload_source_set(hlir.clone())?;
    }

    println!("===== preload decls for input source =====");

    type_resolver.preload_source_set(hlfiles.clone())?;

    println!("===== resolve types =====");
    // resolve types

    let std_hlir: Vec<_> = std_hlir
        .into_iter()
        .map(|s| type_resolver.source_set(s))
        .collect::<Result<_>>()?;

    println!("===== resolve types for input source =====");

    let hlfiles = type_resolver.source_set(hlfiles)?;

    match build_type {
        BuildType::Library => {
            let wlib = WLib::new(hlfiles.clone());
            let wlib_path = out_dir.join(format!("{}.wlib", config.name().unwrap_or_default()));
            wlib.write_to(&wlib_path);
            println!("library written to {}", wlib_path.display());
            return Ok(());
        }
        _ => {}
    };

    println!("===== convert to mlir =====");

    let mut h2m = HLIR2MLIR::new();

    let std_mlir = std_hlir
        .into_iter()
        .map(|w| h2m.convert_from_source_set(w))
        .collect::<Vec<_>>();

    fs::create_dir_all(&mlir_out_dir)?;
    for m in std_mlir.iter() {
        println!("==== {} ====", m.name);
        mlir_out_dir.push(m.name.clone());
        let mut f = fs::File::create(&mlir_out_dir)?;
        write!(f, "{}", m.to_string())?;
        mlir_out_dir.pop();
    }

    let (mlfile, _) = hlir2mlir(hlfiles, &std_mlir, h2m.annotations())?;

    println!("==== {} ====", mlfile.name);
    mlir_out_dir.push(&mlfile.name);
    let mut f = fs::File::create(&mlir_out_dir)?;
    write!(f, "{}", mlfile.to_string())?;
    mlir_out_dir.pop();

    let module_name = &mlfile.name;
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, module_name);

    for m in std_mlir.iter() {
        codegen.file(m.clone());
    }

    codegen.file(mlfile.clone());

    let output = if let Some(output) = output {
        String::from(output)
    } else {
        let mut output_path = Path::new(&mlfile.name).to_path_buf();
        output_path.set_extension("ll");
        String::from(output_path.to_str().unwrap())
    };

    if let Some(target_triple) = config.target_triple() {
        codegen.set_target_triple(target_triple);
    }

    let mut out_path = out_dir;
    out_path.push(output);

    println!("Output Path -> {:?}", out_path);

    codegen.print_to_file(out_path)?;

    Ok(())
}
