pub enum Languages {
    French,
    Malagasy,
    English,
}

pub fn get(lang: String) -> Languages {
    if lang == String::from("malagasy") {
        Languages::Malagasy
    } else if lang == String::from("french") {
        Languages::French
    } else if lang == String::from("english") {
        Languages::English
    } else {
        Languages::Malagasy
    }
}
