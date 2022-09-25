use crate::read_data::read_json;
use rand::Rng;
use std::collections::HashMap;

pub enum GameType {
    French,
    Malagasy,
    English,
}

pub struct Word(String);

impl Word {
    pub fn new(word: &str) -> Self {
        Self(word.to_string())
    }

    fn shuffle(&mut self) {
        let mut shuffled_word = String::new();
        let mut rng = rand::thread_rng();
        while self.0.len() > 0 {
            let index = rng.gen_range(0..self.0.len());
            shuffled_word.push(self.0.remove(index));
        }
        self.0 = shuffled_word.to_uppercase();
    }

    pub fn format(&mut self, separator: Option<char>) -> String {
        let mut word = String::new();
        let char = match separator {
            Some(c) => c,
            _ => ' ',
        };
        self.shuffle();
        for c in self.0.chars() {
            word.push(c);
            word.push(char);
        }
        word.trim_end_matches(char).to_string()
    }
}

pub struct WordGameApp {
    data: Vec<String>,
    pub to_find: Vec<String>,
    pub word: Word,
    response: String,
}

impl WordGameApp {
    pub fn new(game_type: GameType) -> Self {
        let path = match game_type {
            GameType::Malagasy => "src/assets/ohabolana.json",
            GameType::French => "src/assets/mots.json",
            _ => {
                panic!("Not implemented yet");
            }
        };
        let mut _self = Self {
            to_find: vec![],
            word: Word::new(""),
            data: read_json(path),
            response: String::new(),
        };
        _self.get_random();
        _self.get_results();
        _self
    }

    fn get_results(&mut self) {
        let word = self.word.0.to_lowercase().to_string();
        let mut results: Vec<String> = vec![];
        for text in self.data.clone() {
            let mut tmp: HashMap<String, Vec<char>> = HashMap::new();
            for charset in text.chars() {
                if word.contains(charset) {
                    tmp.entry(text.clone()).or_insert(vec![]);
                    let arr = tmp.get_mut(&text).unwrap();
                    if !arr.contains(&charset) {
                        arr.push(charset);
                    }
                    if text != word && text.len() == tmp.get(&text).unwrap().len() {
                        results.push(text.clone());
                    }
                }
            }
        }
        let mut rng = rand::thread_rng();
        let mut i = 0;
        while i < rng.gen_range(0..results.len()) {
            if self.to_find.len() < 3 {
                self.to_find.push(results[i].clone());
            }
            i += 1;
        }
    }

    pub fn check(self, word: String) -> bool {
        println!("{} to find", self.to_find.len());
        self.to_find.contains(&word)
    }

    fn get_random(&mut self) {
        if self.response.is_empty() {
            let mut rng = rand::thread_rng();
            let i = rng.gen_range(0..self.data.len());
            self.word = Word::new(self.data[i].as_str());
            self.response = self.data[i].clone();
        }
    }
}
