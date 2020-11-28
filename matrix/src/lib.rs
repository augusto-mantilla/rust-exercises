// First exercise

// # Instructions
// Define a data structure to represent a matrix of any size and
// implement the basic operations for this you will need to follow the
// next steps:

// We will consider a matrix as a rectangular arrangements of scalars
// Therefore you will have to create the definition of a Scalar by
// defining a trait Scalar in a new file called `scalar.rs`.

// A scalar type must implement the operations
// Addition, Subtraction, Multiplication, Modulo, Division (you might
// also have to use more restrictions). For this use a trait
// inheritance (supertraits)

// Another condition for a number to be a scalar is to have a zero
// (neutral element in the addition) and a one (neutral element in the
// multiplication). Therefore the Scalar trait will require 2
// functions zero() and one()

// After finishing implement the Scalar trait for u32

// Resources: https://doc.rust-lang.org/book/ch19-03-advanced-traits.html

mod scalar;
use scalar::Scalar;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn zero_property() {
		let matrix: Matrix<u32> = Matrix::zero(3, 4);
		let expected: Matrix<u32> =
			Matrix(vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0]]);
		assert_eq!(matrix, expected);

		let matrix: Matrix<u32> = Matrix::zero(2, 2);
		let expected: Matrix<u32> = Matrix(vec![vec![0, 0], vec![0, 0]]);
		assert_eq!(matrix, expected);
	}

	#[test]
	fn identy_matrix() {
		let matrix: Matrix<u32> = Matrix::identity(2);
		let expected: Matrix<u32> = Matrix(vec![vec![1, 0], vec![0, 1]]);
		assert_eq!(matrix, expected);

		let matrix: Matrix<u32> = Matrix::identity(3);
		let expected: Matrix<u32> = Matrix(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]);
		assert_eq!(matrix, expected);
	}
}
