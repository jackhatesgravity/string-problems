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

// Helper function
fn is_vowel(c: char) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    vowels.contains(&c)
}

// Helper function for the Pig Latin operation
fn count_starting_consonants(word: &str) -> usize {
    let mut result = 0;
    for c in word.chars() {
        if is_vowel(c) {
            break;
        } else {
            result += 1;
        }
    }

    result
}

// Pig Latin
pub fn to_pig_latin(word: &str) -> String {
    // Words that begin with consonants: move first letter to end and add -ay.
    // Words that begin with consonant clusters: move cluster to end and add -ay.
    // Words that begin with vowels: add -nay.
    // Single letter words pass right through without conversion.

    // Slightly messy approach, but count number of starting consonants and use match to respond.
    let cluster_size = count_starting_consonants(word);
    match cluster_size {
        0 => println!("{word} starts with a vowel."),
        1 => println!("{word} starts with a consonant."),
        2 => println!("{word} starts with two consonants."),
        3 => println!("{word} starts with three consonants."),
        _ => println!("{word} is not a valid English word."),
    }

    String::from("Hello")
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
    fn checks_if_vowel() {
        assert!(is_vowel('a'))
    }

    #[test]
    fn checks_if_consonant() {
        assert!(!is_vowel('b'))
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

    #[test]
    #[ignore]
    fn converts_from_pig_latin() {
        todo!()
    }


    #[test]
    #[ignore]
    fn converts_to_caesar_cipher() {
        todo!()
    }


    #[test]
    #[ignore]
    fn converts_from_caesar_cipher() {
        todo!()
    }


    #[test]
    #[ignore]
    fn counts_vowels() {
        todo!()
    }

    #[test]
    #[ignore]
    fn identifies_palindrome() {
        todo!()
    }
}