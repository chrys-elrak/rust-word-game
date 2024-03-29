use serde_json::Value;
use std::{
    fs,
    io::{self, Write},
};

use crate::language::Languages;

pub fn read_json(path: &str) -> Vec<String> {
    let mut data: Vec<String> = vec![];
    let content = fs::read_to_string(path);
    match content {
        Ok(content) => {
            let value: Value = serde_json::from_str(&content).unwrap_or_default();
            value.as_array().unwrap().iter().for_each(|x| {
                data.push(x.as_str().unwrap().to_string());
            });
        }
        _ => {}
    }
    data
}

pub fn read_input(prompt_message: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt_message);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_lowercase().to_string()
}

pub fn get_path(lang: Languages) -> &'static str {
    match lang {
        Languages::Malagasy => "src/assets/ohabolana.json",
        Languages::French => "src/assets/mots.json",
        _ => {
            panic!("Not implemented yet");
        }
    }
}