// Define a function that calculate the transpose matrix of a 2x2 matrix
// You don't need to understand everything about matrices

// Just convert line into columns and vice versa
// ( a b )	__ transposition __> ( a d )
// ( c d )  					 ( b d )

// Only the body of the transpose function can be changed
fn main() {
	let matrix = Matrix((1, 3), (4, 5));
	println!("Original matrix {:?}", matrix);
	println!("Transpose matrix {:?}", transpose(matrix));
}

#[derive(PartialEq, Debug)]
struct Matrix((i32, i32), (i32, i32));

fn transpose(m: Matrix) -> Matrix {
	// You're code here
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn it_works() {
		assert_eq!(transpose(Matrix((1, 3), (4, 7))), Matrix((1, 4), (3, 7)));
		assert_eq!(transpose(Matrix((1, 0), (8, 7))), Matrix((1, 8), (0, 7)));
		assert_eq!(transpose(Matrix((3, 3), (4, 2))), Matrix((3, 4), (3, 2)));
		assert_eq!(transpose(Matrix((8, 3), (3, 5))), Matrix((8, 3), (3, 5)));
	}
}
