use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Pointer};
use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

#[derive(Debug)]
struct WizError(String);

impl Display for WizError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl Error for WizError {}

pub(crate) fn create_project(path: &PathBuf, project_name: &str) -> Result<(), Box<dyn Error>> {
    if !path.read_dir()?.next().is_none() {
        return Err(Box::new(WizError(format!(
            "`{}` is not empty",
            path.display()
        ))));
    };
    let mut path = path.clone();
    path.push("Package.wiz");
    let mut package_wiz = BufWriter::new(File::create(&path)?);
    writeln!(
        package_wiz,
        r#"
val package = Package.init(
    name: {:?},
)"#,
        project_name
    )?;
    path.pop();

    path.push("src");
    create_dir_all(&path)?;
    path.push("main.wiz");
    let mut main_wiz = BufWriter::new(File::create(&path)?);
    writeln!(
        main_wiz,
        r#"
fun main() {{
    println("Hello world!")
}}"#
    )?;
    Ok(())
}
