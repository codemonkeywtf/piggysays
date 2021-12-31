pub mod parsed_words {
    pub fn parse(words: Vec<String>) -> Vec<String> {
        let parsed_words: Vec<String> = words
            .iter()
            .map(|w| w.to_string().trim().to_lowercase())
            .collect();
        parsed_words
    }
}
