#[derive(Debug)]
#[derive(Clone)]
pub struct Manifest {
    pub packages: Vec<String>
}

impl Manifest {
    pub fn new() -> Manifest {
        Manifest {
            packages: Vec::from([
                String::from("./packages/*"),
                String::from("./apps/*")
            ])
        }
    }
}