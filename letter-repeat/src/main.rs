// Determine if a word or phrase has a letter that repeats itself or not.
// If the word or phrase repeats one ore more letters, the function will return false
// and if it does not repeat any letter the function will return true.
// However spaces and punctuation are allowed to appear multiple times and an empty string will
// return true.

// Examples:

// lumberjacks      -> true
// Hello            -> false
// There            -> false
// six-year-old     -> true

use std::collections::HashSet;

pub fn letter_repeat(s: &str) -> bool {

}

fn main() {
    println!("{}",letter_repeat("algorithms"));
    println!("{}",letter_repeat("This is false"));

}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn empty_string() {
        assert_eq!(letter_repeat(""), true);
        assert_eq!(letter_repeat("ambidextrously"), true);
        assert_eq!(letter_repeat("uncopyrightable"), true);
        assert_eq!(letter_repeat("thumbscrew-japingly"), true);
        assert_eq!(letter_repeat("Emily Jung Schwartzkopf"), true);
        assert_eq!(letter_repeat("DOCTORWHO"), false);
        assert_eq!(letter_repeat("Alphabet"), false);
        assert_eq!(letter_repeat("eleven"), false);
        assert_eq!(letter_repeat("six-year-old"), true)
    }
}