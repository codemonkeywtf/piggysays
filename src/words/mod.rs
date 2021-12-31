use std::io::stdin;

pub mod parse_words;

pub fn get_words(prompt: String) -> Vec<String> {
    let mut word = String::new();
    println!("{}", prompt);
    stdin()
        .read_line(&mut word)
        .expect("WTF!!! Just type a dang word.");
    let words: Vec<_> = word
        .split(" ")
        .map(|w| w.to_string().trim().to_lowercase())
        .collect();
    words
}
