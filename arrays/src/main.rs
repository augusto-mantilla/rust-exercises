// Define a function call thirtytwo_tens that returns an array with 32
// positions fill with only the value 10:
// [10, 10, 10, ... 10].len() = 32

fn main() {
	let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
	let a1: Vec<i32> = (1..11).collect();
	let b = [5; 10];

	println!("The Sum of the elements in {:?} = {}", a, sum(a));
	println!("The Sum of the elements in {:?} = {}", a, sum_v2(a));
	println!("The Sum of the elements in {:?} = ", a1);
	println!("The Sum of the elements in {:?} = {}", b, sum(b));
	println!(
		"Array size 50 with all elements to 10 = {:?}",
		thirtytwo_tens().len()
	);
}

fn thirtytwo_tens() ->  {
}

// Complete the function sum to return the sum of the all the elements
// in the vector passed as an argument
fn sum() -> i32 {
}

