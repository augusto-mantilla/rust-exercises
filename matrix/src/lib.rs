// # Instructions
// Define a data structure to represent a matrix of any size and
// Hint: You can use wrappers and Vec<T>
// We will consider a matrix as a rectangular arrangements of scalars
// (Use the trait defined in the exercise before (lalgebra_scalar))
// implement the basic functions identity and zero

// ## Matrix::identity
// Matrix::identity(n) must return the identity matrix of the size n
// Ex:
// Matrix::identity(2) == [[1, 0],[0, 1]]
// next steps:

// ## Matrix::zero
// Matrix::zero(row, col) returns the zero matrix of size row x col
// Ex:
// Matrix::zero(3, 2) == [[0,0],[0,0],[0,0]]

// Resources: https://doc.rust-lang.org/book/ch19-03-advanced-traits.html

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
