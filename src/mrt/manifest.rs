#[derive(Debug, Clone)]
pub struct Manifest {
    pub packages: Vec<String>,
}

impl Default for Manifest {
    fn default() -> Self {
        Self::new()
    }
}

impl Manifest {
    pub fn new() -> Manifest {
        Manifest {
            packages: Vec::from([String::from("./packages/*"), String::from("./apps/*")]),
        }
    }
}
