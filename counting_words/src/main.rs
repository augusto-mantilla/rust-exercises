// In this program you will have to create a function `counting_words`, that
// receives a string and returns the number of times that the word appears on the string.

// The program will count as a word the following:
//     A number like ("0" or "1234") will count as 1
//     A simple word or letter like ("a" or "they") will count as 1
//     Two simple words joined by a single apostrophe ("it's" or "they're") will count as 1

// The program must respect the following rules:
//     The count is case insensitive ("HELLO", "Hello", and "hello" are 3 uses of the same word)
//     All forms of punctuation have to be ignored except for the apostrophe if used like the example above.
//     The words can be separated by any form of whitespace (ie "\t", "\n", " ")

use std::collections::HashMap;

/// Count occurrences of words.
fn counting_words(words: &str) -> HashMap<String, u32> {
    words
        .to_lowercase()
        .split(|c: char| !c.is_alphanumeric() && c != '\'')
        .map(|w| w.trim_matches('\''))
        .filter(|w| !w.is_empty())
        .fold(HashMap::new(), |mut map, w| {
            *map.entry(String::from(w)).or_default() += 1;
            map
        })
}

fn main() {
    println!("{:?}", counting_words("Hello, world!"));
    println!("{:?}", counting_words("“Two things are infinite: the universe and human stupidity; and I'm not sure about the universe.”
    ― Albert Einstein "));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        assert_eq!(counting_words("Hello"), &[("Hello", 1)]);
         assert_eq!(counting_words("hello big world"), &[("hello", 1), ("big", 1), ("world", 1)]);
         assert_eq!(counting_words("one of each"), &[("one", 1), ("of", 1), ("each", 1)]);
         assert_eq!(counting_words("HELLO, 1, 2 HELLO"), &[("HELLO", 2), ("1", 1), ("2", 1)]);
         assert_eq!(counting_words(
            "Batman, BATMAN, batman, Stop stop"),
            &[("batman", 3), ("stop", 2)],
        );
         assert_eq!(counting_words(
            " multiple   whitespace"),
            &[("multiple", 1), ("whitespace", 1)],
        );
    }

    #[test]
    #[ignore]
    fn test_count_multiple_occurrences() {
         assert_eq!(counting_words(
            "one fish two fish red fish blue fish"),
            &[("one", 1), ("fish", 4), ("two", 1), ("red", 1), ("blue", 1)],
        );
    }

    #[test]
    #[ignore]
    fn test_multi_lines() {
         assert_eq!(counting_words(
            "Game\nNight\nTomorrow"),
            &[("Game", 1), ("Night", 1), ("Tomorrow", 1)],
        );
    }

    #[test]
    #[ignore]
    fn test_punctuation() {
         assert_eq!(counting_words(
            "keyboard : mouse on the desk : Computer!!&@$%^&"),
            &[
                ("keyboard", 1),
                ("mouse", 1),
                ("on", 1),
                ("the", 1),
                ("desk", 1),
                ("Computer", 1),
            ],
        );
    }

    #[test]
    #[ignore]
    fn with_apostrophes() {
         assert_eq!(counting_words(
            "First: don't laugh. Then: don't cry."),
            &[
                ("first", 1),
                ("don't", 2),
                ("laugh", 1),
                ("then", 1),
                ("cry", 1),
            ],
        );
    }

    #[test]
    #[ignore]
    fn test_apostrophe() {
         assert_eq!(counting_words(
            "Joe can't tell between 'large' and large."),
            &[
                ("joe", 1),
                ("can't", 1),
                ("tell", 1),
                ("between", 1),
                ("large", 2),
                ("and", 1),
            ],
        );
    }
}
