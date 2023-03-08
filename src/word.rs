use rand::Rng;
use unicode_segmentation::UnicodeSegmentation;
pub struct Word(String);

impl Word {
    pub fn new(word: &str) -> Self {
        Self(word.to_string())
    }

    pub fn get_string(&self) -> String {
        self.0.to_lowercase().to_string()
    }

    fn set_string(&mut self, new_value: String) {
        self.0 = new_value;
    }

    pub fn shuffle(&mut self) -> String {
        let mut shuffled = String::new();
        let mut rng = rand::thread_rng();
        let mut copy = self.get_string().to_string();
        while copy.len() > 0 {
            let index = rng.gen_range(0..copy.len());
            shuffled.push(copy.remove(index));
        }
        self.set_string(shuffled.to_uppercase());
        shuffled
    }

    pub fn format(&mut self, separator: Option<char>) -> String {
        let mut word = String::new();
        // Check separator, if None, use space ` ` as default
        let text = self.shuffle(); // shuffle the word
        // format the word with separator
        for (_, c) in text.grapheme_indices(true){
            word.push_str(c);
            word.push(separator.unwrap_or(' '));
        }
        word.to_uppercase()
    }
}
