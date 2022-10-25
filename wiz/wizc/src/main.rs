use crate::high_level_ir::node_id::ModuleId;
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
use wiz_syntax_parser::parser::wiz::read_package_from_path;
use wizc_cli::{BuildType, Config, ConfigExt};
use wizc_message::Message;

mod high_level_ir;
mod llvm_ir;
mod middle_level_ir;

fn get_builtin_find_path() -> PathBuf {
    PathBuf::from_iter([env!("HOME"), ".wiz", "lib", "src"])
}

fn get_find_paths() -> Vec<PathBuf> {
    vec![get_builtin_find_path()]
}

const BUILTIN_LIB: [&str; 3] = ["core", "libc", "std"];

fn main() -> Result<()> {
    println!("{:?}", env::args());
    let app = wizc_cli::app("wizc");
    let matches = app.get_matches();
    let config = Config::from(&matches);

    let mut session = Session::new(config);
    session.timer("compile", |s| run_compiler(s))
}

fn run_compiler(session: &mut Session) -> Result<()> {
    run_compiler_internal(session, false)
}

fn run_compiler_internal(session: &mut Session, no_std: bool) -> Result<()> {
    let output = session.config.output();
    let paths = session.config.paths();
    let out_dir = session
        .config
        .out_dir()
        .unwrap_or_else(|| env::current_dir().unwrap());

    let mlir_out_dir = out_dir.join("mlir");

    let input_source = session.timer::<Result<_>, _>("parse files", |session| {
        read_package_from_path(
            &session.parse_session,
            session.config.input(),
            session.config.name(),
        )
    })?;

    let mut arena = Arena::default();

    let std_hlir = session.timer("load dependencies", |session| {
        let mut libraries = session.config.libraries();

        let std_hlir: Result<Vec<_>> = if libraries.is_empty() && !no_std {
            let find_paths: Vec<_> = get_find_paths().into_iter().chain(paths).collect();

            let mut lib_paths = vec![];

            for lib_name in BUILTIN_LIB {
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
                .map(|(p, name)| read_package_from_path(&session.parse_session, p, Some(*name)))
                .collect::<Result<Vec<_>>>()?;
            Ok(source_sets
                .into_iter()
                .enumerate()
                .map(|(i, s)| ast2hlir(session, &mut arena, s, ModuleId::new(i)))
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
        ast2hlir.lowing(input_source, ModuleId::new(std_hlir.len()))
    })?;

    session.timer("type check", |session| {
        let mut type_checker = TypeChecker::new(session, &arena);
        type_checker.verify(&hlfiles);
    });
    match session.config.type_() {
        BuildType::Library => {
            let wlib = WLib::new(hlfiles);
            let wlib_path = {
                let mut path = out_dir.join(session.config.name().unwrap_or_default());
                path.set_extension("wlib");
                path
            };
            wlib.write_to(&wlib_path);
            println!("{}", Message::output(wlib_path));
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
    let mut codegen = CodeGen::new(
        &context,
        module_name,
        session.config.target_triple().as_deref(),
    );

    for m in std_mlir.into_iter() {
        codegen.file(m);
    }

    codegen.file(mlfile.clone());

    if let Some(emit) = session.config.emit() {
        let output = if let Some(output) = output {
            PathBuf::from(output)
        } else {
            let mut output_path = PathBuf::from(&mlfile.name);
            output_path.set_extension("ll");
            output_path
        };

        let out_path = out_dir.join(output);

        println!("{}", Message::output(&out_path));

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
    use crate::run_compiler_internal;
    use std::path::PathBuf;
    use wiz_session::Session;
    use wizc_cli::{BuildType, Config, ConfigBuilder, ConfigExt};

    struct TestContext {
        manifest_dir: PathBuf,
    }

    impl TestContext {
        fn new() -> Self {
            Self {
                manifest_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
            }
        }

        fn test_resource_dir(&self) -> PathBuf {
            self.manifest_dir.join("resources/").join("test")
        }

        fn repository_root(&self) -> PathBuf {
            self.manifest_dir
                .join("..")
                .join("..")
                .canonicalize()
                .unwrap()
        }

        fn lib_path(&self) -> PathBuf {
            self.repository_root().join("libraries")
        }

        fn out_dir(&self) -> PathBuf {
            self.repository_root().join("out")
        }
    }

    #[test]
    fn compile_file() {
        let context = TestContext::new();
        let target_file_path = context.test_resource_dir().join("helloworld.wiz");

        let config = Config::default()
            .input(target_file_path.to_str().unwrap())
            .path(context.lib_path())
            .out_dir(context.out_dir());
        let mut session = Session::new(config);
        run_compiler(&mut session).unwrap()
    }

    #[test]
    fn compile_ilb_core() {
        let context = TestContext::new();
        let target_lib_path = context.lib_path().join("core").join("src");
        let config = Config::default()
            .input(target_lib_path.to_str().unwrap())
            .name("core")
            .type_(BuildType::Library)
            .out_dir(context.out_dir());
        let mut session = Session::new(config);
        run_compiler_internal(&mut session, true).unwrap();
    }
}
