use crate::app::App;
use crate::language::get;
use crate::read_data::read_input;

pub fn run() {
    let lang = read_input("Make your choice (malagasy/french): ");
    let mut main = App::new(get(lang));
    let find_me = main.word.format(Some('*'));
    loop {
        println!(
            ">>{}<<\nSCORE: {}",
            find_me,
            main.get_score()
        );
        // dbg!(main.get_bonus());
        dbg!(&main.response);
        let user_input = read_input("Enter your word: ");
        main.check_bonus(&user_input.to_lowercase(), 0.5);
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
