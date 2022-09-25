use crate::read_data::read_input;
use crate::word::{GameType::French, WordGameApp};

mod read_data;
mod word;

fn main() {
    // println!("Let' s play game\nYou have to find the word in french\nYou have 5 chances\nGood luck");
    let mut main = WordGameApp::new(French);
    let find_me = main.word.format(None);
    println!("{}, {:?}", find_me, main.to_find);
    let user_input = read_input("Enter your word: ");
    println!("{} => {}", user_input, main.to_find.contains(&user_input));
}
