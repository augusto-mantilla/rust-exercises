// Write a function called `doubtful` that adds to every string passed
// to it a question mark (?)

// You have to fix the code to make it compile an for that you can
// only modify the code where is indicated

fn main() {
	//You can add code in the next line before `s`
	s = String::from("Hello");
	println!("Before changing the string: {}", s);
	doubtful(/*you can add code here*/);
	println!("After changing the string: {}", s);
}

// This function doesn't return anything
fn doubtful(/*you can add code here*/) {
	//Add the body of the function here
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn it_works() {
		let mut s = "hello".to_string();
		let s_copy = s.clone();

		doubtful(&mut s);

		assert_eq!(s, s_copy + "?");
	}
}
