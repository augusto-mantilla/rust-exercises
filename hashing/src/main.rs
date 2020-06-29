// Given a list of integers (Vec<i32>) write three functions
// Write a function called `median` that calculates the `median` (for a sorted list is the value in the middle)
// Write a function called `mode` that calculates the mode (the value
// that appears more often)

use std::collections::HashMap;

fn main() {
	println!("Hello, world!");
	let v = vec![4, 7, 5, 2, 5, 1, 3];
	println!("mean {}", mean(&v));
	println!("median {}", median(&v));
	println!("mode {}", mode(&v));
}

fn mean(list: &Vec<i32>) -> f64 {}

fn median(list: &Vec<i32>) -> i32 {}

fn mode(list: &Vec<i32>) -> i32 {}

#[cfg(test)]
mod test {
	use super::*;
	use std::f64;

	fn approx_eq(a: f64, b: f64) -> bool {
		(a - b).abs() < f64::EPSILON
	}

	#[test]
	fn test_mean() {
		let v = vec![4, 7, 5, 2, 5, 1, 3];
		assert!(approx_eq(mean(&v), 3.857142857142857));
	}

	#[test]
	fn test_median() {
		let v = vec![4, 7, 5, 2, 5, 1, 3];
		assert_eq!(median(&v), 4);
	}

	#[test]
	fn test_mode() {
		let v = vec![4, 7, 5, 2, 5, 1, 3];
		assert_eq!(mode(&v), 5);
	}
}
