use string_problems::{print_to_pig_latin};

fn main() {
    let words = vec!["apple", "title", "thistle", "split"];

    for word in words {
        print_to_pig_latin(word);
    }
}

