use rand::Rng;
use std::collections::HashMap;

use crate::enums::game_type::GameType;
use crate::read_data::{read_input, read_json};
use crate::word::Word;

pub struct App {
    data: Vec<String>,
    bonus: Vec<String>,
    score: f32,
    pub word: Word,
    pub response: String,
}

impl App {
    pub fn set_bonus(&mut self, new_value: Vec<String>) {
        self.bonus = new_value;
    }

    pub fn update_score(&mut self, new_value: f32) {
        self.score += new_value;
    }

    pub fn get_score(&self) -> f32 {
        self.score
    }

    fn check(&mut self, text: &String) -> bool {
        let correct = self
            .bonus
            .iter()
            .map(|x| x.to_lowercase())
            .collect::<Vec<String>>()
            .contains(&text.to_lowercase());
        if correct {
            self.update_score(0.5);
        }
        correct
    }

    pub fn new(game_type: GameType) -> Self {
        let path = match game_type {
            GameType::Malagasy => "src/assets/ohabolana.json",
            GameType::French => "src/assets/mots.json",
            _ => {
                panic!("Not implemented yet");
            }
        };
        let mut _self = Self {
            score: 0.0,
            bonus: vec![],
            word: Word::new(""),
            data: read_json(path),
            response: String::new(),
        };
        _self.generate_random();
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
        self.set_bonus(results);
    }

    #[allow(dead_code)]
    fn update_bonus_word(&self, data: Vec<String>) {
        let mut rng = rand::thread_rng();
        let mut i = 0;
        let mut bonus = Vec::new();
        while i < rng.gen_range(0..data.len()) {
            if bonus.len() < 3 {
                bonus.push(data[i].clone());
            }
            i += 1;
        }
    }

    fn generate_random(&mut self) {
        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0..self.data.len());
        self.word = Word::new(self.data[i].as_str());
        self.response = self.data[i].to_lowercase();
    }

    pub fn reset(&mut self) {
        self.generate_random();
        self.get_results();
    }
}

pub mod game {
    use super::{read_input, App};
    use crate::enums::game_type::parse_game_type;

    pub fn run() {
        let lang = read_input("Choose your lang: ");
        let mut main = App::new(parse_game_type(lang.as_str()));
        let find_me = main.word.format(Some('*'));
        loop {
            println!(
                "{}: {:?} => {}\nSCORE: {}",
                find_me,
                main.bonus,
                main.response,
                main.get_score()
            );
            let user_input = read_input("Enter your word: ");
            if main.check(&user_input) {
                main.set_bonus(
                    main.bonus
                        .iter()
                        .filter(|&x| x != &user_input)
                        .map(|x| x.to_string())
                        .collect(),
                );
            }
            if user_input == main.response {
                main.update_score(1.0);
                println!("You win with {}", main.get_score());
                let user_input = read_input("Continue? (y/n) ");
                if user_input == "n" {
                    break;
                }
                main.reset();
            }
        }
    }
}
