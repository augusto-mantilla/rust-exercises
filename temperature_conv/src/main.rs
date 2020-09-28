// Modify the body of the functions fahrenheit_to_celsius and
// celsius_to_fahrenheit to
// Make this code compile and pass the test
fn main() {
	println!("{} F = {} C", -459.67, fahrenheit_to_celsius(-459.67)); // It should print -459.67 F = -459.67 F = -273.15 C
	println!("{} C = {} F", 0.0, celsius_to_fahrenheit(0.0)); // It should print 0.0 C = 32 F
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
	// You're code here
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
	// You're code here
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::f64::EPSILON;

	fn eql(a: f64, b: f64) -> bool {
		(b - a).abs() < EPSILON
	}

	#[test]
	fn test_f_to_c() {
		assert!(eql(fahrenheit_to_celsius(20.0), -6.666666666666666));
		assert!(eql(fahrenheit_to_celsius(83.0), 28.333333333333332));
	}

	#[test]
	fn test_c_to_f() {
		assert!(eql(celsius_to_fahrenheit(27.0), 80.6));
		assert!(eql(celsius_to_fahrenheit(0.0), 32.0))
	}
}
