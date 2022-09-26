use crate::app::App;
use crate::enums::game_type::parse_game_type;
use crate::read_data::read_input;

pub fn run() {
    let lang = read_input("Make your choice (malagasy/french): ");
    let mut main = App::new(parse_game_type(lang.as_str()));
    let find_me = main.word.format(Some('*'));
    loop {
        println!(
            "{}: {:?} => {}\nSCORE: {}",
            find_me,
            main.get_bonus(),
            main.response,
            main.get_score()
        );
        let user_input = read_input("Enter your word: ");
        if main.check(&user_input) {
            main.remove_from_bonus(user_input.as_str());
        }
        if user_input == main.response {
            main.update_score(1.0);
            println!("You win with {}", main.get_score());
            let user_input = read_input("Continue? (y/n) ");
            if user_input == "n" {
                break;
            }
            main.reset();
        }
    }
}
