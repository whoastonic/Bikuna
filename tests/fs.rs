use std::fs;
use std::path::Path;

use serde::Serialize;
use lib_kuna::file_man;

#[derive(Debug, Serialize)]
struct TestPayload {
	name: String,
	show: String
}

#[test]
fn read_file () {
	if let Ok(contents) = file_man::read(Path::new("tests/data/read-file.txt")) {
		assert_eq!(&contents, "Hello, World!\n")
	} else {
		panic!("Failed to read content path")
	}
}

#[test]
fn write_to_file () {
	let path = Path::new("tests/data/test_payload.json");
	fs::remove_file(&path).unwrap_or_default();

	let payload = serde_json::to_vec_pretty(&TestPayload {
		name: String::from("Akiza"),
		show: String::from("5Ds"),
	}).expect("cannot create test payload");


	assert!(file_man::write(path, &payload).is_ok())
}
