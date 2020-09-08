// Write a function, called `open_or_create` that tries to open a file
// and if it doesn't exist creates it and returns it

fn main() {
	open_or_create("hello.txt");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_open_or_create() {
		open_or_create("hello.txt");
	}
}
