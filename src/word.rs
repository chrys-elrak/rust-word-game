use rand::{seq::SliceRandom};
pub struct Word(String);

impl Word {
    pub fn new(word: &str) -> Self {
        Self(word.to_string())
    }

    pub fn get_string(&self) -> &str {
        &self.0
    }

    fn set_string(&mut self, new_value: &str) {
        self.0 = new_value.to_string();
    }

    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        let mut word: Vec<_> = self.get_string().chars().collect();
        word.shuffle(&mut rng);
        let shuffled: String = word.into_iter().collect();
        self.set_string(shuffled.as_str());
    }

    pub fn format(&mut self, separator: Option<char>) -> String {
        let mut word = String::new();
        let char = match separator {
            Some(c) => c,
            _ => ' ',
        };
        self.shuffle();
        for c in self.get_string().chars() {
            word.push(c);
            word.push(char);
        }
        word.trim_end_matches(char).to_string()
    }
}
