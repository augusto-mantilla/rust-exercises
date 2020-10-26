// Build a program called talking that allows you to talk with
// your computer. His responses will be very limited and you will create them.

// The computer answers "Let me check" if you ask him a question, such as "How old are you?".
// He answers "There is no need to yell, calm down!" if you yell at him, for example "HOW ARE YOU?"
// (it is consider yelling wen the sentence is all written in capital letters).
// He answers "Quiet, I know what I"m doing!" if you yell a question at him.
// He says "Just say something!" if you address him without actually saying anything.
// He answers "Interesting" to anything else.

pub fn talking(text: &str) -> &str {
}

fn main() {
    println!("{:?}", talking("JUST DO IT!"));
    println!("{:?}", talking("Hello how are you?"));
    println!("{:?}", talking("WHAT'S GOING ON?"));
    println!("{:?}", talking("something"));
    println!("{:?}",talking(""));
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_yell() {
        assert_eq!(talking("JUST DO IT!"), "There is no need to yell, calm down!");
        assert_eq!(talking("1, 2, 3 GO!"), "There is no need to yell, calm down!");
        assert_eq!(talking("I LOVE YELLING"), "There is no need to yell, calm down!");
        assert_eq!(talking("WJDAGSAF ASVF EVA VA"), "There is no need to yell, calm down!");

    }

    #[test]
    fn test_question() {
        assert_eq!(talking("Hello how are you?"), "Sure.");
        assert_eq!(talking("Are you going to be OK?"), "Sure.");
        assert_eq!(talking("7?"), "Sure.");
        assert_eq!(talking("Like 15?"), "Sure.");
    }

    #[test]
    fn test_question_yelling() {
        assert_eq!(talking("WHAT'S GOING ON?"), "Quiet, I know what I'm doing!");
        assert_eq!(talking("ARE YOU FINISHED?"), "Quiet, I know what I'm doing!");
        assert_eq!(talking("WHAT DID I DO?"), "Quiet, I know what I'm doing!");
        assert_eq!(talking("ARE YOU COMING?"), "Quiet, I know what I'm doing!");
    }

    #[test]
    fn test_interesting() {
        assert_eq!(talking("something"), "Interesting");
        assert_eq!(talking("Wow that's good!"), "Interesting");
        assert_eq!(talking("Run far"), "Interesting");
        assert_eq!(talking("1 2 3 go!"), "Interesting");
        assert_eq!(talking("This is not ? a question."), "Interesting");
    }

    #[test]
    fn test_empty() {
        assert_eq!(talking(""), "Just say something!");
        assert_eq!(talking("										"), "Just say something!");
        assert_eq!(talking("          "), "Just say something!");
    }
}

