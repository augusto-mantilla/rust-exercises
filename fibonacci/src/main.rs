// Complete the body of the function fibonacci so the this functions
// receives a number n and returns the nth number in the fibonacci series.

// The Fibonacci Series starts like this: 0, 1, 1, 2, 3, 5, 8, 13...
// so fibonacci(2) = 1
// and fibonnacci(4) = 3

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
