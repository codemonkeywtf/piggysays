use std::env;

mod character;
// mod piglatin;
mod words;

use crate::character::wilbur;
use crate::words::get_words;
use crate::words::parse_words::parsed_words::parse;
use crate::words::piglatin::piggy_says::piggy_latin;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            let words: Vec<String> = get_words(String::from("Enter a word or phrase: "));
            println!();
            piggy_latin(words);
        }
        _ => {
            let words: Vec<String> = parse(args[1..].to_vec());
            println!();
            piggy_latin(words);
        }
    }

    wilbur();
}
