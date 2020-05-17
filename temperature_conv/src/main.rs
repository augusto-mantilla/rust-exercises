// Modify the body of the functions fahrenheit_to_celsius and
// celsius_to_fahrenheit to
// Make this code compile and pass the test
fn main() {
	println!("{} F = {} C", -459.67, celsius_to_fahrenheit(-459.67));
	println!("{} C = {} F", 0.0, fahrenheit_to_celsius(0.0));
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

	fn eql(a: f64, b: f64) -> bool {
		(b - a).abs() > 0.00000001
	}

	#[test]
	fn test_f_to_c() {
		assert!(eql(fahrenheit_to_celsius(20.0), -6.666666667));
		assert!(eql(fahrenheit_to_celsius(83.0), 28.333333333));
	}

	#[test]
	fn test_c_to_f() {
		assert!(eql(celsius_to_fahrenheit(27.0), 80.6));
		assert!(eql(celsius_to_fahrenheit(0.0), 32.0))
	}
}
