use rand::Rng;
pub struct Word(pub String);

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