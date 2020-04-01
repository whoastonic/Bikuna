use super::types::{BikunaError, BikunaResult};

use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn read(name: &str) -> BikunaResult<String> {
    let path = Path::new(name);

    match File::open(path) {
        Ok(mut file) => {
            let mut contents = String::new();

            match file.read_to_string(&mut contents) {
                Ok(_) => BikunaResult::Ok(contents),
                Err(err) => Err(BikunaError::FileSys(err.to_string())),
            }
        }
        Err(err) => Err(BikunaError::FileSys(err.to_string())),
    }
}
