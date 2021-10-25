use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Pointer};
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
    Ok(())
}
