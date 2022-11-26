#[cfg(test)]
pub mod utils {
    use std::{path::PathBuf, env};

    pub fn get_repo_root() -> PathBuf {
        let mut path = env::current_dir().unwrap();
        while !path.join(".git").exists() {
            path = path.parent().unwrap().to_path_buf();
        }
        path
    }
}