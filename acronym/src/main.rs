// Write a function `acronym` that turns a phrase into an acronym.
// Example:
//          "HyperText Markup Language" -> HTML
//          "Something - I made up" -> "SIMU"
//          "GNU Image Manipulation Program" -> "GIMP"

pub fn acronym(phrase: &str) -> String {

}

fn main() {
        println!("{}",acronym("Hello, world!"));
    }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiple_cases() {
        assert_eq!(acronym(""), "");
        assert_eq!(acronym("Portable Network Graphics"), "PNG");
        assert_eq!(acronym("HyperText Markup Language"), "HTML");
        assert_eq!(acronym("Ruby on Rails"), "ROR");
        assert_eq!(acronym("First In, First Out"), "FIFO");
        assert_eq!(acronym("GNU Image Manipulation Program"), "GIMP");
        assert_eq!(acronym("PHP: Hypertext Preprocessor"), "PHP");
        assert_eq!(
            acronym("Complementary metal-oxide semiconductor"),
            "CMOS"
        );
        assert_eq!(
            acronym("Rolling On The Floor Laughing So Hard That My Dogs Came Over And Licked Me"),
            "ROTFLSHTMDCOALM"
        );
        assert_eq!(
            acronym("Something - I made up from thin air"),
            "SIMUFTA"
        );
        assert_eq!(
            acronym("The Road _Not_ Taken"),
            "TRNT"
        );
        assert_eq!(
            acronym("Halley's Comet"),
            "HC"
        );
    }
}