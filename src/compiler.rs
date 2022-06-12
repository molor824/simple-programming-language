use std::fs::read_to_string;

pub struct Compiler {
    source: String,
}
impl Compiler {
    pub fn new(path: &str) -> Result<Self, std::io::Error> {
        match read_to_string(path) {
            Ok(str) => Ok(Self { source: str }),
            Err(e) => Err(e),
        }
    }
    pub fn compile(&self) {}
}
