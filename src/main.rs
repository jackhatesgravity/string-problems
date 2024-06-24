use string_problems::to_pig_latin;

fn main() {
    let word = to_pig_latin("apple");
    let word = word.unwrap();
    println!("{}", word);
    let word = to_pig_latin("title");
    let word = word.unwrap();
    println!("{}", word);
    let word = to_pig_latin("thistle");
    let word = word.unwrap();
    println!("{}", word);
    let word = to_pig_latin("split");
    let word = word.unwrap();
    println!("{}", word);
}
