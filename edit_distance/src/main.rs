// Create a function call `edit_distance` that calculates the minimum
// number of changes (insertion, deletions and substitutions) that
// need to be made to a string `source` to arrive to another `target`
// string

// For more information and examples https://en.wikipedia.org/wiki/Edit_distance

fn edit_distance(source: &str, target: &str) -> usize {}

fn main() {
	let source = "alignment";
	let target = "assignment";
	println!(
		"It's necessary to make {} change(s) to {}, to get {}",
		edit_distance(source, target),
		source,
		target
	);
	//The edit distance of the example should be 2
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_distance() {
		assert_eq!(edit_distance("gumbo", "gambol"), 2);
		assert_eq!(edit_distance("kitten", "sitting"), 3);
		assert_eq!(edit_distance("rosettacode", "raisethysword"), 8);
	}
}
