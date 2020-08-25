//Create the function `bigger` that compares two i32 numbers
//and returns the largest of those numbers
// You can not use:
// - another function call

fn main() {
    println!("The biggest of the elements is {}", bigger(45, 432));
	println!("The biggest of the elements is {}", bigger(1000, 40000));
	println!("The biggest of the elements is {}", bigger(60, 13));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn five_is_bigger_than_one() {
        assert_eq!(5, bigger(5, 1));
    }

    #[test]
    fn fifty_is_bigger_than_thirty() {
        assert_eq!(50, bigger(30, 50));
    }
}
