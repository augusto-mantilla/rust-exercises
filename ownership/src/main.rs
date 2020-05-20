// Create a function that returns the length of a string without
// taking ownership of the value

fn main() {
	let s = String::from("Hello");
	let l = str_len(&s);
	println!("\tlen(\"{}\") = {}", s, l);
}

fn str_len(s: &str) -> usize {
	// Write you're code here
}
