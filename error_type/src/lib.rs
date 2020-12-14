/*
## error types

### Instructions

For this exercise you will have to implement an **error type**.

The main objective is to create a form validation, where you must implement an
error type. This must validate the password and the first name. The
first name must not be empty and the password must have at least 8 characters and a combination of alphanumeric and none alphanumeric ASCII characters

ex: "asDd123=%" => good
    "asgfD"     => error
    "asdsdf2"   => error
    "sad_#$"    => error

Create a structure called `Form` that will have the following fields:

- `first_name`, that will be a string
- `last_name`, that will be also a string
- `birth`, of type [`NaiveDate`](https://docs.rs/chrono/0.4.19/chrono/naive/struct.NaiveDate.html) that will convert a string "2015-09-05" to a date of that format
- `sex`, `SexType` that must be an `enum` with the fields `Male` and `Female`
- `birth_location`, that will be a string
- `password`, that will be a string

You must also implement for this structure a function to initialize the structure, `new` and a function called
`validate` that will validate the form by checking the previous constrains.
Otherwise it will return a vector with of string literals `["Valid first name", "Valid password"]`

For the error type you must create a type struct called `FErr`, that will be the type for the error.
It must have the fields:

- `form_values`, this will be a tuple of strings that will save the value that the user inserted into the form

ex: ("password", "asdaSD_") or
    ("first_name", "someone")

- `date`, that will have the date that the error occurred in the format "2020-12-14 09:33:41"
- `err`, that will have the error description:
    - "No user name"
    - "At least 8 characters"
    - "Combination of different ASCII character types (numbers, letters and none alphanumeric characters)"

### Example

```rust
fn main() {
    let mut form_output = Form::new(
        String::from("Jack"),
        String::from("Sparrow"),
        create_date("2015-09-05"),
        SexType::Male,
        String::from("Africa"),
        String::from("qwqQWsa1dty_"));

    println!("{:?}", form_output);
    // output:
    // Form { first_name: "Jack", last_name: "Sparrow", birth: 2015-09-05, sex: Male), birth_location: "Africa"), password: "qwqQWsa1dty_"}

    println!("{:?}", form_output.validate().unwrap());
    // output:
    // ["Valid first name", "Valid password"]
    
    form_output.first_name = String::from("");
    println!("{:?}", form_output.validate().unwrap_err());
    // output:
    // FErr { form_values: ("first_name", ""), date: "2020-12-11 18:37:08", err: "No user name" }

    form_output.password = String::from("dty_1");

    println!("{:?}", form_output.validate().unwrap_err());
    // output:
    // FErr { form_values: ("password", "dty_1"), date: "2020-12-11 18:49:31", err: "At least 8 characters" }

    form_output.password = String::from("asdasASd(_");

    println!("{:?}", form_output.validate().unwrap_err());
    // output:
    // FErr { form_values: ("password", "asdasASd(_"), date: "2020-12-11 18:51:10", err: "Combination of different ASCII character types (numbers, letters and am none alphanumeric characters)" }

    form_output.password = String::from("asdasASd123SA");

    println!("{:?}", form_output.validate().unwrap_err());
    // output:
    // FErr { form_values: ("password", "asdasASd123SA", date: "2020-12-11 18:51:10", err: "Combination of different ASCII character types (numbers, letters and am none alphanumeric characters)" }
}
```

### Notions

- https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/define_error_type.html
- https://docs.rs/chrono/0.4.19/chrono/naive/struct.NaiveDate.html

*/
// this will be the structure that wil handle the errors
#[derive(Debug, Eq, PartialEq)]
struct FErr {}

#[derive(Debug, Eq, PartialEq)]
enum SexType {}

#[derive(Debug, Eq, PartialEq)]
struct Form {
    first_name: String,
    last_name: String,
    birth: NaiveDate,
    sex: SexType,
    birth_location: String,
    password: String,
}


#[cfg(test)]
mod tests {
    use super::*;

    fn create_date(date: &str) -> NaiveDate {
        NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap()
    }

    #[derive(Debug)]
    struct TestForm<'a> {
        form: Form,
        validation: Result<Vec<&'a str>, FErr>
    }

    impl <'a> TestForm<'_> {
        // all test cases
        fn new() -> Vec<TestForm<'a>> {
            vec![
                TestForm {
                    form : Form::new(
                    String::from("Katy"),
                    String::from("Silva"),
                    create_date("2015-09-05"),
                    SexType::Female,
                    String::from("Africa"),
                    String::from("qwTw12&%$3sa1dty_")),
                    validation: Ok(vec!["Valid first name", "Valid password"]),
                },
                TestForm {
                    form : Form::new(
                    String::from(""),
                    String::from("Bear"),
                    create_date("2015-09-05"),
                    SexType::Male,
                    String::from("Africa"),
                    String::from("qwTw12&%$3sa1dty_")),
                    validation: Err(FErr {
                        form_values: (String::from("first_name"),
                        String::from("")),
                        date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                        err: String::from("No user name")}),
                },
                TestForm {
                    form : Form::new(
                    String::from("Someone"),
                    String::from("Bear"),
                    create_date("2015-09-05"),
                    SexType::Male,
                    String::from("Africa"),
                    String::from("12345")),
                    validation: Err(FErr {
                        form_values: (String::from("password"), String::from("12345")),
                        date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                        err: String::from("At least 8 characters") }),
                },
                TestForm {
                    form : Form::new(
                    String::from("Someone"),
                    String::from("Bear"),
                    create_date("2015-09-05"),
                    SexType::Male,
                    String::from("Africa"),
                    String::from("sdASDsrW")),
                    validation: Err(FErr {
                        form_values: (String::from("password"), String::from("sdASDsrW")),
                        date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                        err: String::from("Combination of different ASCII character types (numbers, letters and none alphanumeric characters)") }),
                },
                TestForm {
                    form : Form::new(
                    String::from("Someone"),
                    String::from("Bear"),
                    create_date("2015-09-05"),
                    SexType::Female,
                    String::from("Africa"),
                    String::from("dsGE1SAD213")),
                    validation: Err(FErr {
                        form_values: (String::from("password"), String::from("dsGE1SAD213")),
                        date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                        err: String::from("Combination of different ASCII character types (numbers, letters and none alphanumeric characters)") }),
                },
                TestForm {
                    form : Form::new(
                    String::from("Someone"),
                    String::from("Bear"),
                    create_date("2015-09-05"),
                    SexType::Female,
                    String::from("Africa"),
                    String::from("dsaSD&%DF!?=")),
                    validation: Err(FErr {
                        form_values: (String::from("password"), String::from("dsaSD&%DF!?=")),
                        date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                        err: String::from("Combination of different ASCII character types (numbers, letters and none alphanumeric characters)") }),
                }
            ]
        }
    }

    #[test]
    fn test_error_type() {
        let form_cases = TestForm::new();

        for v in form_cases {
            assert_eq!(v.form.validate(), v.validation);
        }
    }
}