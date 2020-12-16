// Now define the matrix multiplication by implementing the
// std::ops::Mul for the type matrix

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn get_row() {
		let matrix: Matrix<u32> = Matrix(vec![vec![3, 6], vec![8, 0]]);
		assert_eq!(vec![3u32, 6], matrix.row(0));
		assert_eq!(vec![8u32, 0], matrix.row(1));
	}

	#[test]
	fn get_col() {
		let matrix: Matrix<u32> = Matrix(vec![vec![3, 6], vec![8, 0]]);
		assert_eq!(matrix.col(0), vec![3u32, 8]);
		assert_eq!(vec![6u32, 0], matrix.col(1));
	}

	#[test]
	fn matrix_multiplication() {
		let matrix_1: Matrix<u32> = Matrix(vec![vec![0, 1], vec![0, 0]]);
		let matrix_2: Matrix<u32> = Matrix(vec![vec![0, 0], vec![1, 0]]);
		let expected: Matrix<u32> = Matrix(vec![vec![1, 0], vec![0, 0]]);
		assert_eq!(matrix_1 * matrix_2, Some(expected));

		let matrix_1: Matrix<u32> = Matrix(vec![vec![0, 1], vec![0, 0]]);
		let matrix_2: Matrix<u32> = Matrix(vec![vec![0, 0, 0], vec![1, 0, 0], vec![1, 1, 1]]);
		assert_eq!(matrix_1 * matrix_2, None);
	}
}
