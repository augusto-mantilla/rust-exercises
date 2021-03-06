/*
## borrowing_or_not_to_borrow

### Instructions

Ownership is Rust's most unique feature, and it enables Rust to make memory safety guarantees without
needing garbage collector. Therefore you must understand ownership in rust.

Create the following functions :

  - `delete_and_backspace`, imagine that `-` represents the backspace key and the `+` represents the
    delete key, this function must receive a borrowed string and turn this string into the string without
    the backspaces and the deletes. Example: `delete_and_backspace("bpp--o+erroi-+cw") // output: "borrow"`
  - `is_correct` that borrows a Vector of string literals with some correct and incorrect math equations
    and replaces the correct equations with `✔` and the wrong with `✘` and returns a `usize` with the percentage
    of correct equations.


### Example

```rust
fn main() {
    let mut a = String::from("bpp--o+er+++sskroi-++lcw");
    let mut b: Vec<&str> = vec!["2+2=4", "3+2=5", "10-3=3", "5+5=10"];
    
    // - If a value does **not implement Copy**, it must be **borrowed** and so will be passed by **reference**.
    delete_and_backspace(&mut a); // the reference of  the value
    let per = is_correct(&mut b); // the reference of  the value

    println!("{:?}", (a, b, per));
    // output: ("borrow", ["✔", "✔", "✘", "✔"], 75)
}
```

### Notions

- https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html
- https://docs.rs/meval/0.2.0/meval/

*/
use meval::eval_str;

// - If a value does **not implement Copy**, it must be **borrowed** and so will be passed by **reference**.
fn delete_and_backspace(s: &mut String) {

}

// - If a value does **not implement Copy**, it must be **borrowed** and so will be passed by **reference**.
fn is_correct(v: &mut Vec<&str>) -> usize {

}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn reference_string() {
        let mut a_1 = String::from("bpp--o+er+++sskroi-++lcw");
        let mut a_2 = String::from("hs-+deasdasd------l+++dsdp");
        let mut a_3 = String::from("pad-rtic+eulqw--+rar");
        delete_and_backspace(&mut a_1);
        delete_and_backspace(&mut a_2);
        delete_and_backspace(&mut a_3);
        assert_eq!(a_1, "borrow".to_string());
        assert_eq!(a_2, "help".to_string());
        assert_eq!(a_3, "particular".to_string());
    }
    #[test]
    fn reference_vec() {
        let mut b_1: Vec<&str> = vec!["2+2=4", "3+2=5", "10-3=3", "5+5=10"];
        let mut b_2: Vec<&str> = vec!["1+2=4", "0+2=5", "10-3=3", "41+5=10"];
        let mut b_3: Vec<&str> = vec!["2+2=4", "3+2=5", "10-3=7", "5+5=10"];
        let result_1 = is_correct(&mut b_1);
        let result_2 = is_correct(&mut b_2);
        let result_3 = is_correct(&mut b_3);
        assert_eq!(result_1, 75);
        assert_eq!(result_2, 0);
        assert_eq!(result_3, 100);

        assert_eq!(b_1, vec!["✔", "✔", "✘", "✔"]);
        assert_eq!(b_2, vec!["✘", "✘", "✘", "✘"]);
        assert_eq!(b_3, vec!["✔", "✔", "✔", "✔"]);
    }
}

