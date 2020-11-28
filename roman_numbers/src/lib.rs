// Write a function that receives an u32 number and converts it into a
// roman number in subtractive notation (the common way to write roman
// number)

// For this start by defining the digits as RomanDigit with the values
// I, V, X, L, C, D, M and Nulla for 0

// Next define RomanNumber as a wrapper to a vector of RomanDigit's
// And implement the Trait From<u32> to u32 values convert into RomanNumber's

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		assert_eq!(RomanNumber::from(3).0, [I, I, I]);
		assert_eq!(RomanNumber::from(6).0, [V, I]);
		assert_eq!(RomanNumber::from(15).0, [X, V]);
		assert_eq!(RomanNumber::from(30).0, [X, X, X]);
		assert_eq!(RomanNumber::from(150).0, [C, L]);
		assert_eq!(RomanNumber::from(200).0, [C, C]);
		assert_eq!(RomanNumber::from(600).0, [D, C]);
		assert_eq!(RomanNumber::from(1500).0, [M, D]);
	}

	#[test]
	fn substractive_notation() {
		assert_eq!(RomanNumber::from(4).0, [I, V]);
		assert_eq!(RomanNumber::from(44).0, [X, L, I, V]);
		assert_eq!(RomanNumber::from(3446).0, [M, M, M, C, D, X, L, V, I]);
		assert_eq!(RomanNumber::from(9).0, [I, X]);
		assert_eq!(RomanNumber::from(94).0, [X, C, I, V]);
	}
}
