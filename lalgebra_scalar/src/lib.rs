// # Instructions
// A scalar type must implement the operations
// Addition, Subtraction, Multiplication and Division (you might
// also have to use more restrictions). For this use a trait
// inheritance (supertraits)

// Another condition for a number to be a scalar is to have a zero
// (neutral element in the addition) and a one (neutral element in the
// multiplication). Therefore the Scalar trait will require 2
// functions zero() and one()

// After finishing implement the Scalar trait for u32
// And for f32

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn scalar() {
		let a: u32 = u32::zero();
		assert_eq!(a, 0 as u32);

		let b = u32::one();
		assert_eq!(b, 1 as u32);
	}

	#[test]
	fn f32_scalar() {
		let zero = f32::zero();
		assert_eq!(zero, 0.0);
		let one = f32::one();
		assert_eq!(one, 1.0);
	}
}
