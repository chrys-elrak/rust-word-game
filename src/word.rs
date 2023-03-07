use rand::Rng;
use unicode_segmentation::UnicodeSegmentation;
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
    /*
        Shuffle the position of the letters in the word
        It mutate the word and return the new word
    */
    pub fn shuffle(&mut self) -> String {
        let mut shuffled = String::new();
        let mut rng = rand::thread_rng();
        let mut copy = self.get_string().to_string();
        while copy.len() > 0 {
            let index = rng.gen_range(0..copy.len());
            shuffled.push(copy.remove(index));
        }
        self.set_string(shuffled.to_uppercase().as_str());
        shuffled
    }

    pub fn format(&mut self, separator: Option<char>) -> String {
        let mut word = String::new();
        // Check separator, if None, use space ` ` as default
        let char = match separator {
            Some(c) => c,
            _ => ' ',
        };
        let text = self.shuffle(); // shuffle the word
        // format the word with separator
        for c in text.graphemes(true) {
            word.push_str(c);
            word.push_str(c);
        }
        word.trim_end_matches(char).to_string() // remove the last separator
    }
}
