// In this exercise you will define the basic operations with a matrix
// starting by implementing the `std::ops::Add` trait

// Define the operation + (by defining the trait std::ops::Add) for
// two matrices remember that two matrices can only be added if they
// have the same size. Therefore the add method must handle the
// possibility of failure by returning an Option<T>

// And do the equivalent also for the operator `-` (Subtraction).

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_addition() {
		let matrix = Matrix(vec![vec![1, 1], vec![1, 1]]);
		let matrix_2 = Matrix(vec![vec![1, 1], vec![1, 1]]);
		let expected = Matrix(vec![vec![2, 2], vec![2, 2]]);
		assert_eq!(matrix + matrix_2, Some(expected));

		let matrix = Matrix(vec![vec![1, 1], vec![1, 1]]);
		let matrix_2 = Matrix(vec![vec![1, 1, 3], vec![1, 1]]);
		assert_eq!(matrix + matrix_2, None);
	}

	#[test]
	fn test_subtraction() {
		let matrix = Matrix(vec![vec![1, 1], vec![1, 1]]);
		let matrix_2 = Matrix(vec![vec![1, 1], vec![1, 1]]);
		let expected = Matrix(vec![vec![0, 0], vec![0, 0]]);
		assert_eq!(matrix - matrix_2, Some(expected));

		let matrix = Matrix(vec![vec![1, 1], vec![1, 1]]);
		let matrix_2 = Matrix(vec![vec![1, 1, 3], vec![1, 1]]);
		assert_eq!(matrix - matrix_2, None);
	}
}
