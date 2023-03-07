pub enum GameType {
    French,
    Malagasy,
    English,
}

pub fn parse_game_type(game_type: String) -> GameType {
    if game_type == String::from("malagasy") {
        GameType::Malagasy
    } else if game_type == String::from("french") {
        GameType::French
    } else if game_type == String::from("english") {
        GameType::English
    } else {
        GameType::Malagasy
    }
}
