use crate::app::App;
use crate::language::get;
use crate::utils::read_input;

use colored::Colorize;

pub fn run() {
    let lang = read_input(format!("{}", "Make your choice (malagasy/french): ".green()).as_str());
    let mut main = App::new(get(lang));
    let find_me = main.word.format(Some('*'));
    loop {
        println!(
            ">>{}<<\nSCORE: {}",
            find_me.on_yellow(),
            main.get_score()
        );
        let user_input = read_input(format!("{}", "Your answer: ".yellow()).as_str());
        let correct = main.check_bonus(&user_input.to_lowercase(), 0.5);
        if correct {
            println!("{}", "You find a bonus word".on_green());
        }
        if user_input == main.response {
            main.update_score(1.0);
            println!("You win with {}", main.get_score());
            let user_input = read_input(format!("{}\n", "Do you want to play again? (y/n)".underline()).as_str());
            if user_input == "n" {
                break;
            }
            main.reset();
        }
    }
}
