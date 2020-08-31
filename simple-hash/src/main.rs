// Create the function `contain` that checks a `HashMap` to see if it contains the given key.
// Create the function `remove` that removes a given key from the `HashMap`.


use std::collections::HashMap;

fn contain(mut h: HashMap<&str, i32>, s: &str) -> bool {
    match h.get(s) {
        Some(hash) => true,
        None => false,
    }
}
fn remove(mut h: HashMap<&str, i32>, s: &str) {
    h.remove(s);
}

fn main() {
    let mut hash: HashMap< &str, i32> = HashMap::new();
    hash.insert("Daniel", 122);
    hash.insert("Ashley", 333);
    hash.insert("Katie", 334);
    hash.insert("Robert", 14);

    println!(
        "Does the HashMap contains the name Roman? => {}",
        contain(hash, "Roman")
    );
    println!("Removing Robert {:?}", remove(hash, "Robert"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains() {
        #[derive(Debug, Copy, Clone)]
        let mut s = HashMap::new();

        s.insert("Pedro", 43);
        s.insert("Ralph", 12);
        s.insert("Johnny", 546);
        s.insert("Albert", 12323214);

        #[derive(Debug, Copy, Clone)]
        let x = s;

        assert_eq!(true, contain(x, "Pedro"));
        assert_eq!(true, contain(x, "Ralph"));
        assert_eq!(true, contain(x, "Johnny"));
        assert_eq!(true, contain(x, "Albert"));
    }
}
