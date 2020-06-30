// Write a function that tries to open a file and if it doesn't exist
// creates it and returns it
use std::fs::File;

fn main() {
	open_or_create("hello.txt");
}

fn open_or_create(filename: &str) -> File {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_open_or_create() {
		open_or_create("hello.txt");
	}
}
