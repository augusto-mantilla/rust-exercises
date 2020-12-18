/*
## error types

### Instructions

Write a function that tries to open a file and panics if the file
doesn't exist

*/

use std::fs::{self, File};

fn open_file(s: &str) -> File {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	#[should_panic]
	fn test_opening() {
		open_file("file.txt");
	}

	#[test]
	fn test_opening_existing() {
		let filename = "created.txt";
		File::create(filename).unwrap();
		open_file(filename);
		fs::remove_file(filename).unwrap();
	}
}
