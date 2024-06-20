use serde::Deserialize;
use serde_yaml::Value;
use std::fs;
use regex::Regex;
use std::collections::HashMap;
use teloxide::types::{KeyboardMarkup, KeyboardButton};

#[derive(Debug, Deserialize)]
pub struct Command {
    pub name: String,
    pub action: String,
    pub error_msg: String,
}

pub fn load_commands_with_emoji(file_path: &str) -> Vec<Command> {
    let yaml_data = fs::read_to_string(file_path).expect("Unable to read YAML file");
    let all_commands: HashMap<String, HashMap<String, Value>> = serde_yaml::from_str(&yaml_data).expect("Unable to parse YAML");

    let emoji_regex = Regex::new(r"^\p{Emoji}\s*").unwrap();

    all_commands.into_iter()
        .filter(|(key, _)| emoji_regex.is_match(key))
        .map(|(key, value)| {
            let action = emoji_regex.replace(&key, "").to_string().to_lowercase().replace(" ", "_");
            Command {
                name: key.clone(),
                action,
                error_msg: value.get("error_msg").expect("Error message not found").as_str().expect("Error message should be a string").to_string(),
            }
        })
        .collect()
}

pub fn get_main_keyboard(commands: &[Command]) -> KeyboardMarkup {
    let keyboard_buttons: Vec<Vec<KeyboardButton>> = commands.iter()
        .map(|command| &command.name)
        .collect::<Vec<&String>>()
        .chunks(3)
        .map(|chunk| chunk.iter().map(|label| KeyboardButton::new(label.to_string())).collect())
        .collect();

    KeyboardMarkup::new(keyboard_buttons).resize_keyboard(true)
}

pub fn extract_command_from_label(label: &str) -> String {
    Regex::new(r"^\p{Emoji}\s*").unwrap()
        .replace(label, "")
        .to_string()
        .trim()
        .to_lowercase()
        .replace("_", "")
}
