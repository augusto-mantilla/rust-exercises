// Make the following program compile and pass the test, you're only
// allowed to change the body of the function
fn main() {
	let val: u32 = 20;
	println!("Fibonacci({}) = {}", val, fibonacci(val));
}

fn fibonacci(n: u32) -> u32 {
	// Write here
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		assert_eq!(fibonacci(0), 0);
		assert_eq!(fibonacci(1), 1);
		assert_eq!(fibonacci(22), 17711);
		assert_eq!(fibonacci(20), 6765);
	}
}
