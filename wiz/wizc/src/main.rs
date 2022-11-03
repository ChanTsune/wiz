use crate::high_level_ir::ast2hlir;
use crate::high_level_ir::node_id::ModuleId;
use crate::high_level_ir::type_checker::TypeChecker;
use crate::high_level_ir::wlib::WLib;
use crate::llvm_ir::codegen::CodeGen;
use crate::middle_level_ir::hlir2mlir;
use dirs::home_dir;
use inkwell::context::Context;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};
use wiz_arena::Arena;
use wiz_result::Result;
use wiz_session::Session;
use wiz_syntax_parser::parser::wiz::read_book_from_path;
use wizc_cli::{BuildType, Config, ConfigExt, Emit};
use wizc_message::Message;

mod high_level_ir;
mod llvm_ir;
mod middle_level_ir;

fn get_builtin_find_path() -> PathBuf {
    let mut home = home_dir().unwrap();
    home.extend([".wiz", "lib", "src"]);
    home
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
    let paths = session.config.paths();
    let out_dir = session
        .config
        .out_dir()
        .unwrap_or_else(|| env::current_dir().unwrap());

    let mlir_out_dir = out_dir.join("mlir");

    let input_source = session.timer::<Result<_>, _>("parse files", |session| {
        read_book_from_path(
            &session.parse_session,
            session.config.input(),
            session.config.name(),
        )
    })?;

    let mut arena = Arena::default();

    let std_hlir = session.timer("load dependencies", |session| {
        let mut libraries = session.config.libraries();

        if libraries.is_empty() && !no_std {
            let find_paths: Vec<_> = get_find_paths().into_iter().chain(paths).collect();

            let mut lib_paths = vec![];

            for lib_name in BUILTIN_LIB {
                for p in find_paths.iter() {
                    let lib_path = p.join(lib_name);
                    let package_manifest_path = lib_path.join("Package.wiz");
                    if package_manifest_path.exists() {
                        println!("`{}` found at {}", lib_name, lib_path.display());
                        lib_paths.push((lib_path.join("src").join("lib.wiz"), lib_name));
                        break;
                    }
                }
            }
            let mut libs = vec![];
            for (lib_path, name) in lib_paths.iter() {
                let out_dir = env::temp_dir().join(name);
                fs::create_dir_all(&out_dir)?;
                lib::run_compiler_for_std(lib_path, name, &out_dir, &libs)?;
                libs.push({
                    let mut path = out_dir.join(name);
                    path.set_extension("wlib");
                    path
                });
            }
            libraries.extend(libs);
        };
        let std_hlir: Result<Vec<_>> = libraries
            .iter()
            .map(|p| {
                let lib = WLib::read_from(p);
                lib.apply_to(&mut arena)?;
                Ok(lib.typed_ir)
            })
            .collect::<Result<_>>();
        std_hlir
    })?;

    let hlfiles = session.timer("resolve type", |session| {
        ast2hlir(
            session,
            &mut arena,
            input_source,
            ModuleId::new(std_hlir.len()),
        )
    })?;

    session.timer("type check", |session| {
        let mut type_checker = TypeChecker::new(session, &arena);
        type_checker.verify(&hlfiles);
    });
    if let BuildType::Library = session.config.type_() {
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

    codegen.file(mlfile);

    let output =
        session.config.name().map(PathBuf::from).unwrap_or_else(|| {
            PathBuf::from(session.config.input().file_stem().unwrap_or_default())
        });

    let mut out_path = out_dir.join(output);
    match session.config.emit() {
        Emit::LlvmIr => {
            out_path.set_extension("ll");
            codegen.print_to_file(&out_path)
        }
        Emit::Assembly => {
            out_path.set_extension("asm");
            codegen.write_as_assembly(&out_path)
        }
        Emit::Object => {
            out_path.set_extension("o");
            codegen.write_as_object(&out_path)
        }
        Emit::Binary => {
            let mut ir_file = out_path.clone();
            ir_file.set_extension("ll");
            codegen.print_to_file(&ir_file)?;

            Command::new("clang")
                .args(&[ir_file.as_os_str(), "-o".as_ref(), out_path.as_os_str()])
                .output()?;
            Ok(())
        }
    }?;
    println!("{}", Message::output(&out_path));
    Ok(())
}

mod lib {
    use crate::run_compiler_internal;
    use std::path::{Path, PathBuf};
    use wiz_result::Result;
    use wiz_session::Session;
    use wizc_cli::{BuildType, Config, ConfigBuilder};

    pub(crate) fn run_compiler_for_std(
        input: &Path,
        name: &str,
        out_dir: &Path,
        libraries: &[PathBuf],
    ) -> Result<()> {
        let config = Config::default()
            .input(input)
            .name(name)
            .type_(BuildType::Library)
            .out_dir(out_dir)
            .libraries(libraries);
        let mut session = Session::new(config);
        run_compiler_internal(&mut session, true)
    }
}

#[cfg(test)]
mod tests {
    use super::run_compiler;
    use std::path::{Path, PathBuf};
    use wiz_session::Session;
    use wizc_cli::{Config, ConfigBuilder, Emit};

    struct TestContext {
        manifest_dir: PathBuf,
        extra_out: PathBuf,
    }

    impl TestContext {
        fn new() -> Self {
            Self {
                manifest_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
                extra_out: PathBuf::new(),
            }
        }

        fn extra_out<P: AsRef<Path>>(mut self, extra_out: P) -> Self {
            self.extra_out = PathBuf::from(extra_out.as_ref());
            self
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
        let context = TestContext::new().extra_out("binary");
        let target_file_path = context.test_resource_dir().join("helloworld.wiz");

        let config = Config::default()
            .input(target_file_path)
            .path(context.lib_path())
            .out_dir(context.out_dir());
        let mut session = Session::new(config);
        run_compiler(&mut session).unwrap();

        assert!(context.out_dir().join("helloworld").exists())
    }

    #[test]
    fn compile_file_to_ir() {
        let context = TestContext::new().extra_out("llvmir");
        let target_file_path = context.test_resource_dir().join("helloworld.wiz");

        let config = Config::default()
            .input(target_file_path)
            .path(context.lib_path())
            .out_dir(context.out_dir())
            .emit(Emit::LlvmIr);
        let mut session = Session::new(config);
        run_compiler(&mut session).unwrap();

        assert!(context.out_dir().join("helloworld.ll").exists())
    }

    #[test]
    fn compile_file_to_obj() {
        let context = TestContext::new().extra_out("object");
        let target_file_path = context.test_resource_dir().join("helloworld.wiz");

        let config = Config::default()
            .input(target_file_path)
            .path(context.lib_path())
            .out_dir(context.out_dir())
            .emit(Emit::Object);
        let mut session = Session::new(config);
        run_compiler(&mut session).unwrap();

        assert!(context.out_dir().join("helloworld.o").exists())
    }
}
