// Hello, world!
pub fn say_hello() -> String {
    String::from("Hello, world!")
}

// Reverse String



// Pig Latin



// Caesar Cipher



// Vowel Count



// Palindrome Checker



// Advanced - Text Editor

mod tests {
    use super::*;

    #[test]
    fn says_hello() {
        let expect = String::from("Hello, world!");
        assert_eq!(expect, say_hello())
    }

    #[test]
    fn reverses_string() {
        todo!()
    }

    #[test]
    fn converts_to_pig_latin() {
        todo!()
    }

    #[test]
    fn converts_from_pig_latin() {
        todo!()
    }


    #[test]
    fn converts_to_caesar_cipher() {
        todo!()
    }


    #[test]
    fn converts_from_caesar_cipher() {
        todo!()
    }


    #[test]
    fn counts_vowels() {
        todo!()
    }

    #[test]
    fn identifies_palindrome() {
        todo!()
    }

}