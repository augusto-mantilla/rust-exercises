/*
## handling

### Instructions

Write a function, called `open_or_create` that as two arguments:
- `file : &str` which is the name of the files
- `content: &str` being the content to be written into the file

This functions should try to open a file, if it doesn't exist creates it and returns it.
You should panic, with the error, in case something goes wrong.

### Notions

- https://doc.rust-lang.org/std/io/enum.ErrorKind.html
- https://doc.rust-lang.org/std/fs/struct.File.html
*/

fn main() {
	let path = "a.txt";
	File::create(path).unwrap();
   open_or_create(path, "content to be written");

   let mut file = File::open(path).unwrap();

   let mut s = String::new();
   file.read_to_string(&mut s).unwrap();
   println!("{}", s);
   // output: content to be written
}

fn open_or_create(s: &str, content: &str) {
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;
	use std::panic;

	fn get_file_content(filename: &str) -> String {
		let mut file = File::open(filename).unwrap();
		let mut s = String::new();
		file.read_to_string(&mut s).unwrap();
		return s;
	}

	#[test]
	fn test_if_file_exists() {
		let filename = "test_existing_file.txt";
		let content = "hello world!";
		File::create(filename).unwrap();
		open_or_create(filename, content);

		assert_eq!(content, get_file_content(filename));
		fs::remove_file(filename).unwrap();
	}

	#[test]
	fn test_create_file() {
		let file = "no_existing_file.txt";
		let content = "hello world!";
		open_or_create(file, content);

		assert_eq!(content, get_file_content(file));
		fs::remove_file(file).unwrap();
	}
	#[test]
	fn test_error_case() {
		let filename = "hello.txt";
		File::create(filename).unwrap();
		let mut perms = fs::metadata(filename).unwrap().permissions();
		perms.set_readonly(true);
		fs::set_permissions(filename, perms).unwrap();

		let result = panic::catch_unwind(|| open_or_create(filename, "test"));
		assert!(result.is_err());
		fs::remove_file(filename).unwrap();
	}
}
