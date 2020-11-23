// In this exercise a number between 0 and 1000000 will be generated.
// Your purpose is to create the function `spell` that will spell the numbers generated.
// So, if the program generates the number:

// 1 your function will return the string "one"
// 14 your function will return the string "fourteen".
// 29 your function will return the string "twenty-nine".
// 45 your function will return the string "forty-five"
// 96 your function will return the string "ninety-six"
// 100 your function will return the string "one hundred".
// 101 your function will return the string "one hundred and one"
// 348 your function will return the string "one hundred and twenty-three"
// 1002 your function will return the string "one thousand and two".
// 1323 your function will return the string "one thousand three hundred and twenty-three".
// 642616 your function will return the string "six hundred and forty-two thousand six hundred and sixteen"
// 810000 your function will return the string "eight hundred and ten thousand"
// 1000000 your function will return the string "one million"

use rand::Rng;

pub fn spell(n: u64) -> String {

}

fn main() {
    let mut rng = rand::thread_rng();
    println!("{}", spell(rng.gen_range(0, 1000000)));
    println!("{}", spell(rng.gen_range(0, 1000000)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(spell(0), String::from("zero"));
        assert_eq!(spell(1), String::from("one"));
        assert_eq!(spell(14), String::from("fourteen"));
        assert_eq!(spell(20), String::from("twenty"));
        assert_eq!(spell(22), String::from("twenty-two"));
        assert_eq!(spell(101), String::from("one hundred and one"));
        assert_eq!(spell(120), String::from("one hundred and twenty"));
        assert_eq!(spell(123), String::from("one hundred and twenty-three"));
        assert_eq!(spell(1000), String::from("one thousand"));
        assert_eq!(spell(1055), String::from("one thousand and fifty-five"));
        assert_eq!(
            spell(1234),
            String::from("one thousand two hundred and thirty-four")
        );
        assert_eq!(
            spell(10123),
            String::from("ten thousand one hundred and twenty-three")
        );
        assert_eq!(
            spell(910112),
            String::from("nine hundred and ten thousand one hundred and twelve")
        );
        assert_eq!(
            spell(651123),
            String::from("six hundred and fifty-one thousand one hundred and twenty-three")
        );

        assert_eq!(spell(810000), String::from("eight hundred and ten thousand"));
        assert_eq!(spell(1000000), String::from("one million"));
    }
}
