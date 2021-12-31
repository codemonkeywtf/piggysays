pub mod piggy_says {
    pub fn piggy_latin(words: Vec<String>) {
        for word in words {
            let (first, last) = split_first(word.as_str());
            match first {
                "a" | "e" | "i" | "o" | "u" => print!("{}-hay ", word),
                _ => print!("{}-{}ay ", last, first),
            }
        }
    }

    fn split_first(s: &str) -> (&str, &str) {
        match s.chars().next() {
            Some(c) => s.split_at(c.len_utf8()),
            None => s.split_at(0),
        }
    }
}
