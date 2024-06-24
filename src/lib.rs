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
pub fn reverse_string(word: &str) -> Option<String> {
    if word.is_empty() {
        return None;
    }
    let result = word
        .graphemes(true)
        .rev()
        .collect();

    Some(result)
}

fn count_starting_consonants(word: &str) -> usize {
    word.chars()
        .take_while(|&c| !is_vowel(c))
        .count()
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
}

/*
 To Pig Latin function.

 Words that begin with consonants: move first letter to end and add -ay.
 Words that begin with consonant clusters: move cluster to end and add -ay.
 Words that begin with vowels: add -nay.
 Words that start with clusters of more than three aren't valid English words.
 String validation is handled outside of this method.

 I think I need to move the return value to a Result<Option<String>, Err> so I can better handle
 the errors that come up in the encryption process.
 */
pub fn to_pig_latin(word: &str) -> Option<String> {

    // Handle the case of the empty string.
    if word.is_empty() {
        return None;
    }

    // Find the length of the starting consonant cluster.
    let cluster_size = count_starting_consonants(word);

    // Handle the case of a starting vowel.
    if cluster_size == 0 {
        return Some(format!("{}nay", word));
    }

    // Panic for now. Handle better later.
    if cluster_size > 3 {
        panic!("Largest English starting cluster is 3!");
    }

    // Format and return the result.
    let (first_letters, suffix) = word.split_at(cluster_size);
    Some(format!("{}{}ay", suffix, first_letters))
}


// Caesar Cipher


// Vowel Count


// Palindrome Checker


// Advanced - Text Editor

#[cfg(test)]
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
        assert_eq!(String::from("olleh"), reverse_string("hello").unwrap());
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
        assert_eq!(String::from("atinlay"), to_pig_latin("latin").unwrap())
    }

    #[test]
    fn converts_to_pig_latin_double_cluster() {
        assert_eq!(String::from("ingstray"), to_pig_latin("string").unwrap())
    }

    #[test]
    fn converts_to_pig_latin_triple_cluster() {
        assert_eq!(String::from("itsplay"), to_pig_latin("split").unwrap())
    }

    #[test]
    fn converts_to_pig_latin_vowel() {
        assert_eq!(String::from("omelettenay"), to_pig_latin("omelette").unwrap())
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