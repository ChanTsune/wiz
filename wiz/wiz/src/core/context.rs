use std::env;
use std::path::PathBuf;
use dirs::home_dir;

pub struct WizContext;

impl WizContext {
    pub fn home() -> PathBuf {
        let env_wiz_home = env::var_os("WIZ_HOME");
        if let Some(wiz_home) = env_wiz_home {
            return PathBuf::from(wiz_home);
        }
        let home_dir = home_dir().unwrap();
        home_dir.join(".wiz")
    }

    pub fn git_dir() -> PathBuf {
        Self::home().join("git")
    }
}
