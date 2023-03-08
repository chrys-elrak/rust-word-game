mod utils;
mod language;
mod word;
mod app;
mod game;

use colored::Colorize;

fn main() {
    let welcome = "Let' s play game\nYou have to find the word in french\nYou have 5 chances\nGood luck".red();
    println!("{}", welcome);
    game::run();
}
