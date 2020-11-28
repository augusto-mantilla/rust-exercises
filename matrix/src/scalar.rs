// Second exercise

// Continuing with this library we will define the Matrix<T> type
// we will define it as a wrapper for a Vec of Vecs (To make it flexible)
// That is, as a two dimensional Vec

// Let's start to define functions for our matrices

// First define an associated function call `new` that returns an
// empty matrix
// then define the associated function zero(row, col) that returns
// a matrix of size `row x col` (row by col) with all positions filled
// with the zero of each type

// Then define the function identity that returns the identity matrix
// of size `row x col`

#[cfg(test)]
mod test {
	#[test]
	fn scalar() {
		let a: u32 = u32::zero();
		assert_eq!(a, 0 as u32);

		let b = u32::one();
		assert_eq!(b, 1 as u32);
	}
}
