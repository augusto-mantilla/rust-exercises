// Complete the signature and the body of the `str_len` function so it
// receives a string or a string literal and returns its length (of
// type usize)
// without taking ownership of the value

fn main() {
	let s = "hello";
	let s1 = "hello".to_string();

	println!("\tstr_len(\"{}\") = {}", s, str_len(s));
	println!("\tstr_len(\"{}\") = {}", s1, str_len(&s1));
}

fn str_len() {
	// Write you're code here
}

// Modify the function first_subword so that takes ownership of a string and returns the
// first sub-word in it. It should work for camelCase as well as snake_case
// first_subword(camelCase) returns camel
// first_subword(snake_case) returns snake

fn first_subword(s: ) -> String {
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	// maybe not the best way to make the test, but I wanted to use
	// lifetimes
	fn str_len_test() {
		struct TstLit<'a> {
			str: &'a str,
			l: usize,
		}

		struct TstString {
			str: String,
			l: usize,
		}

		let tsts = vec![
			TstLit { str: "hello", l: 5 },
			TstLit { str: "how", l: 3 },
			TstLit {
				str: "are you",
				l: 7,
			},
			TstLit {
				str: "change",
				l: 6,
			},
		];
		let o_tsts = vec![
			TstString {
				str: "hello".to_string(),
				l: 5,
			},
			TstString {
				str: "how".to_string(),
				l: 3,
			},
			TstString {
				str: "are you".to_string(),
				l: 7,
			},
			TstString {
				str: "change".to_string(),
				l: 6,
			},
		];

		for t in tsts.iter() {
			assert_eq!(t.l, str_len(t.str));
		}

		for t in o_tsts.iter() {
			assert_eq!(t.l, str_len(&t.str));
		}
	}

	#[test]
	fn first_subword_test() {
		struct TstString<'a> {
			str: String,
			l: &'a str,
		}

		let o_tsts = vec![
			TstString {
				str: "helloWorld".to_string(),
				l: "hello",
			},
			TstString {
				str: "how_you".to_string(),
				l: "how",
			},
			TstString {
				str: "Changeyou".to_string(),
				l: "Changeyou",
			},
			TstString {
				str: "CamelCase".to_string(),
				l: "Camel",
			},
		];

		for t in o_tsts.iter() {
			assert_eq!(t.l.to_string(), first_subword(t.str.clone()));
		}
	}
}
