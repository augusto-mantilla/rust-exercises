// Fix the declaration of the struct called Person so the code compiles
// The type of the fields cannot change (name is &str and age is u8)
// and create the associated function new which creates a new person
// with age 0 and with the name passed as an argument

struct Person {
	name: &str,
	age: u8,
}

fn main() {
	let person = Person::new("Leo");

	println!("Person = {:?}", person);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn fields() {
		let person = Person {
			name: "Dijkstra",
			age: 10,
		};
		assert_eq!(person.age, 10);
		assert_eq!(person.name, "Dijkstra");
	}

	#[test]
	fn create_person() {
		let person = Person::new("Leo");
		assert_eq!(person.age, 0);
		assert_eq!(person.name, "Leo");
	}
}
