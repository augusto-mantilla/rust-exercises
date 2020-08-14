// Define a tuple to represent a student
// Each student is identified by an id number of type i32
// His/her name and his/her last name as a string
// Print the tuple to stdout

struct Student();

fn main() {
	let student = /* id, first_name, last_name */
	println!("Student: {}", );
	// Print only the first name
	println!("Student first name: {}",);
	// Print only the last name
	println!("Student last name: {}",);
	// Print only the id
	println!("Student Id: {}",);
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn it_works() {
		let student = Student(20, String::from("Pedro"), String::from("Domingos"));
		assert_eq!(student.0, 20);
		assert_eq!(student.1, "Pedro".to_string());
		assert_eq!(student.2, "Domingos".to_string());
	}
}
