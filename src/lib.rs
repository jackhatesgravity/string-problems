extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

// Hello, world!
pub fn greeting_string() -> String {
    String::from("Hello, world!")
}

pub fn greeting_slice() -> &'static str {
    "Hello, world!"
}

// Reverse String (using graphemes, not chars)
pub fn reverse_string(word: &str) -> String {
    let result = word
        .graphemes(true)
        .rev()
        .collect();
    result
}

// Helper function for the Pig Latin operation
pub fn count_starting_consonants(word: &str) -> usize {
    todo!()
}

// Pig Latin
pub fn to_pig_latin(word: &str) -> String {
    // Early out for singletons
    if word.len() < 2 {
        return String::from(word);
    }

    // I think a good way to approach this is to use grapheme masks, so that's what I'm doing.
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    if word.starts_with(vowels) {
        println!("Starts with vowel!");
    }

    // Words that begin with consonants: move first letter to end and add -ay.
    // Words that begin with consonant clusters: move cluster to end and add -ay.
    // Words that begin with vowels: add -nay.
    // Single letter words pass right through without conversion.
    String::from("Hello")

    // Already I know this needs to be moved to a match pattern, using if for now
}


// Caesar Cipher


// Vowel Count


// Palindrome Checker


// Advanced - Text Editor

mod tests {
    use super::*;

    #[test]
    fn says_hello_by_string() {
        assert_eq!(String::from("Hello, world!"), greeting_string())
    }

    #[test]
    fn says_hello_by_slice() {
        assert_eq!("Hello, world!", greeting_string())
    }

    #[test]
    fn reverses_string() {
        assert_eq!(String::from("olleh"), reverse_string("hello"));
    }

    #[test]
    fn counts_starting_consonants_single() {
        assert_eq!(1, count_starting_consonants("today"))
    }

    #[test]
    fn counts_starting_consonants_double() {
        assert_eq!(2, count_starting_consonants("thursday"))
    }

    #[test]
    fn counts_starting_consonants_triple() {
        assert_eq!(3, count_starting_consonants("split"))
    }

    #[test]
    fn counts_starting_consonants_none() {
        assert_eq!(0, count_starting_consonants("always"))
    }

    #[test]
    fn converts_to_pig_latin_consonant() {
        assert_eq!(String::from("atinlay"), to_pig_latin("latin"))
    }

    #[test]
    fn converts_to_pig_latin_double_cluster() {
        assert_eq!(String::from("ingstray"), to_pig_latin("string"))
    }

    fn converts_to_pig_latin_triple_cluster() {
        assert_eq!(String::from("itsplay"), to_pig_latin("split"))
    }

    #[test]
    fn converts_to_pig_latin_vowel() {
        assert_eq!(String::from("omeletteway"), to_pig_latin("omelette"))
    }

    #[test]
    fn converts_to_pig_latin_singleton() {
        assert_eq!(String::from("a"), to_pig_latin("a"))
    }

    // #[test]
    // fn converts_from_pig_latin() {
    //     todo!()
    // }
    //
    //
    // #[test]
    // fn converts_to_caesar_cipher() {
    //     todo!()
    // }
    //
    //
    // #[test]
    // fn converts_from_caesar_cipher() {
    //     todo!()
    // }
    //
    //
    // #[test]
    // fn counts_vowels() {
    //     todo!()
    // }
    //
    // #[test]
    // fn identifies_palindrome() {
    //     todo!()
    // }
}