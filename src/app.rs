use rand::Rng;
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

use crate::language::Languages;
use crate::word::Word;

pub struct App {
    data: Vec<String>,
    bonus_words: Vec<String>,
    score: f32,
    pub word: Word,
    pub response: String,
}

impl App {
    fn set_bonus(&mut self, new_value: Vec<String>) {
        self.bonus_words = new_value;
    }

    fn remove_from_bonus(&mut self, value: &str) {
        self.bonus_words.retain(|x| x != value);
    }

    pub fn update_score(&mut self, new_value: f32) {
        self.score += new_value;
    }

    pub fn get_score(&self) -> f32 {
        self.score
    }

    pub fn check_bonus(&mut self, text: &String, bonus_value: f32) -> bool {
        let correct = self
            .bonus_words
            .iter()
            .map(|x| x.to_lowercase())
            .collect::<Vec<String>>()
            .contains(&text);
        if correct {
            self.update_score(bonus_value);
            self.remove_from_bonus(text.as_str());
        }
        correct
    }

    pub fn new(game_type: Languages) -> Self {
        let path = super::utils::get_path(game_type);
        let mut self_instance = Self {
            score: 0.0,
            bonus_words: vec![],
            word: Word::new(""),
            data: super::utils::read_json(path),
            response: String::new(),
        };
        self_instance.generate_random();
        self_instance.get_bonus();
        self_instance
    }

    fn get_bonus(&mut self) {
        let word = self.word.get_string();
        let mut results: Vec<String> = vec![];
        for text in self.data.iter() {
            let mut tmp: HashMap<String, Vec<&str>> = HashMap::new();
            for charset in text.graphemes(true) {
                if word.contains(charset) {
                    tmp.entry(text.clone()).or_insert(vec![]);
                    let arr = tmp.get_mut(text).unwrap();
                    if !arr.contains(&charset) {
                        arr.push(charset);
                    }
                    if text.eq(&word) && text.len() == tmp.get(text).unwrap().len() {
                        results.push(text.clone());
                    }
                }
            }
        }
        self.set_bonus(results);
    }

    fn generate_random(&mut self) {
        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0..self.data.len());
        self.response = self.data[i].to_lowercase();
        self.word = Word::new(self.data[i].as_str());
        self.word.shuffle();
    }

    pub fn reset(&mut self) {
        self.generate_random();
        self.get_bonus();
    }
}
