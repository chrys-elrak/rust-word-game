use app::{game as Game};

mod read_data;
mod enums;
mod word;
mod app;

fn main() {
    println!("Let' s play game\nYou have to find the word in french\nYou have 5 chances\nGood luck");
    Game::run();
}
