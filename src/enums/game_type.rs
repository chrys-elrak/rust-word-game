pub enum GameType {
    French,
    Malagasy,
    English,
}

pub fn parse_game_type(game_type: &str) -> GameType {
    match game_type {
        "malagasy" => GameType::Malagasy,
        "french" => GameType::French,
        "english" => GameType::English,
        _ => GameType::Malagasy,
    }
}
