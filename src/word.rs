use rand::Rng;
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
        let mut shuffled_word = String::new();
        let mut rng = rand::thread_rng();
        while self.get_string().len() > 0 {
            let index = rng.gen_range(0..self.get_string().len());
            shuffled_word.push(self.get_string().to_string().remove(index));
        }
        self.set_string(shuffled_word.to_uppercase().as_str());
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
