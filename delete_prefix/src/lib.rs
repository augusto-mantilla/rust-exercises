// Define the function `delete_prefix(prefix: &str, s: &str) -> Option<&str>`
// That takes 2 slices of string and returns the string of slice s
// with the `prefix` removed wrapped in Some
// If `prefix ` is not contained in `s` return None

// Example:
// delete_prefix("hello, ", "hello, world")? == "world"
// delete_prefix("not", "win");

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_delete_prefix() {
		assert_eq!(
			delete_prefix("augusto", "augusto ornelas"),
			Some(" ornelas")
		);

		assert_eq!(delete_prefix("ab", "b"), None);

		assert_eq!(delete_prefix("aa", "ab"), None);

		assert_eq!(delete_prefix("á©", "á©ab"), Some("ab"));
	}
}
