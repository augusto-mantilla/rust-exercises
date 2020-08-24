// Using your knowledge of generics
// create a structure called `Types` with the ability to store any type of values

#[derive(Debug)]
struct Types<T> {
    value: T,
}

impl<T> Types<T> {
    pub fn new(value: T) -> Self {
        Types { value }
    }
}

fn main() {
    let a = "hello";
    let b = 21;
    let c = Types { value: a };
    println!("{:?}", c);
    let d = Types { value: b };
    println!("{:?}", d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrapper_u32() {
        assert_eq!(Types::new(42).value, 42);
    }

    #[test]
    fn wrapper_str() {
        assert_eq!(Types::new("Foo").value, "Foo");
    }

    #[test]
    fn wrapper_f64() {
        assert_eq!(Types::new(1.00).value, 1.00);
    }

    #[test]
    fn wrapper_vec() {
        assert_eq!(Types::new(1.00).value, 1.00);
    }
}