use std::path::Path;
use std::fs::OpenOptions;
use std::io::{self, Read, Write};

type ManResult<T> = io::Result<T>;

pub fn write(path: &Path, bytes: &[u8]) -> ManResult<()> {
	match OpenOptions::new().write(true).create(true).open(path) {
		Ok(mut file) => {
			file.write_all(bytes)?;
			Ok(())
		}
		Err(error) => Err(build_error_message(error, None)),
	}
}

pub fn read(path: &Path) -> ManResult<String> {
	match OpenOptions::new().read(true).open(path) {
		Ok(mut file) => {
			let mut contents = String::new();

			file.read_to_string(&mut contents)?;

			Ok(contents)
		}
		Err(error) => Err(build_error_message(error, None)),
	}
}

fn build_error_message(err: io::Error, msg: Option<&str>) -> io::Error {
	let def_errmsg = format!("unhandled error: {:?}", err.kind());

	io::Error::new(
		err.kind(),
		if let Some (msg) = msg {
			msg
		} else {
			&def_errmsg
		}
	)
}
