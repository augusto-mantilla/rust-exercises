// Create a struct called Person that has two fields: name of type
// string slice (&str) and age of type u8
// and create the associated function new which creates a new person
// with age 0 and with the name given

#[derive(Debug)]
struct Person {
	name: &str,
	age: u8,
}

impl Person {
	fn new(name: &str) -> {
	}
}

fn main() {
	let person = Person::new("Leo");

	println!("Person = {:?}", person);
}
