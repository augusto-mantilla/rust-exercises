// Write a **program** that transforms a string passed as argument in its `Pig Latin` version.

// The rules used by Pig Latin are as follows:

// - If a word begins with a vowel, just add "ay" to the end.
// - If it begins with a consonant, then we take all consonants before the first vowel and we put them on the end of the word and add "ay" at the end.
// - If the word has no vowels the program should print "No vowels".

fn pig_latin(text: &str) -> String {}

fn main() {
    println!("{}", pig_latin("igloo"));
    println!("{}", pig_latin("apple"));
    println!("{}", pig_latin("hello"));
    println!("{}", pig_latin("chair"));
    println!("{}", pig_latin("xenon"));
    println!("{}", pig_latin("school"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_beginning_with_vowel() {
        assert_eq!(pig_latin("apple"), "appleay");
        assert_eq!(pig_latin("ear"), "earay");
        assert_eq!(pig_latin("igloo"), "iglooay");
        assert_eq!(pig_latin("object"), "objectay");
        assert_eq!(pig_latin("under"), "underay");
    }

    #[test]
    fn test_no_vowel() {
        assert_eq!(pig_latin("ppk rhj sf"), "No vowels");
    }

    #[test]
    fn test_word_beginning_with_consonant() {
        assert_eq!(pig_latin("pig"), "igpay");
        assert_eq!(pig_latin("koala"), "oalakay");
        assert_eq!(pig_latin("yellow"), "ellowyay");
        assert_eq!(pig_latin("xenon"), "enonxay");
        assert_eq!(pig_latin("qat"), "atqay");
        assert_eq!(pig_latin("chair"), "airchay");
        assert_eq!(pig_latin("therapy"), "erapythay");
        assert_eq!(pig_latin("thrush"), "ushthray");
        assert_eq!(pig_latin("school"), "oolschay");
    }
}
