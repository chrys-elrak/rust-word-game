mod read_data;
mod enums;
mod word;
mod app;
mod game;

fn main() {
    println!("Let' s play game\nYou have to find the word in french\nYou have 5 chances\nGood luck");
    game::run();
}
