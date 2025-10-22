use std::collections::HashSet;
use std::sync::atomic::Ordering;

use super::player::Player;
use super::declarations::{SCRIPT_GENERATION_ERROR, WHINGE_MODE};

pub type PlayConfig = Vec<(String, String)>; // (character_name, part_file_name)

pub struct Play {
    play_title: String,
    play_characters: Vec<Player>
}

const CONFIG_LINE_TOKEN_COUNT: usize = 2;
const CHARACTER_NAME_INDEX: usize = 0;
const PART_FILE_NAME_INDEX: usize = 1;
const MINIMUM_CONFIGURATION_LINES: usize = 2;
const TITLE_CONFIGURATION_INDEX: usize = 0;
const FIRST_CHARACTER_CONFIGURATION_INDEX: usize = 1;
const CONFIGURATION_INDEX_STEP: usize = 1;
const FIRST_CHARACTER_LINE: usize = 0;
const CHARACTER_LINE_STEP: usize = 1;
const MAXIMUM_LINES_PER_LINE_NUMBER: usize = 1;
const MAXIMUM_CHARACTERS_PER_LINE_NUMBER: usize = 1;

impl Play {

    fn new() -> Play {
        Play { play_title: String::new(), play_characters: Vec::new() }
    }

    pub fn process_config(&mut self, config: &PlayConfig) -> Result<(), u8> {
        
        for config_entry in config {
            match config_entry {
                (part_name, part_file_name) => {
                    let mut play_character = Player::new(part_name);
                    if let Err(_error_code) = play_character.prepare(part_file_name) {
                        return Err(SCRIPT_GENERATION_ERROR);
                    }
                    self.play_characters.push(play_character);
                }
            }
        }

        Ok(())

    }

    pub fn add_config(config_line: &String, config: &mut PlayConfig) {

        let tokens: Vec<&str> = config_line.split_whitespace().collect();

        if tokens.len() < CONFIG_LINE_TOKEN_COUNT && WHINGE_MODE.load(Ordering::SeqCst) {
            println!("Warning: Configuration line has too few tokens (expected {}, got {}): '{}'", CONFIG_LINE_TOKEN_COUNT, tokens.len(), config_line);
        } else if tokens.len() > CONFIG_LINE_TOKEN_COUNT && WHINGE_MODE.load(Ordering::SeqCst) {
            println!("Warning: Configuration line has too many tokens (expected {}, got {}): '{}'", CONFIG_LINE_TOKEN_COUNT, tokens.len(), config_line);
        }

        if tokens.len() >= CONFIG_LINE_TOKEN_COUNT {
            config.push((tokens[CHARACTER_NAME_INDEX].to_string(), tokens[PART_FILE_NAME_INDEX].to_string()));
        }

    }

    pub fn read_config(config_filename: &String, title: &mut String, config: &mut PlayConfig) -> Result<(), u8> {

        let mut config_lines: Vec<String> = Vec::new();

        if let Err(_error_code) = super::script_gen::grab_trimmed_file_lines(config_filename, &mut config_lines) {
            return Err(SCRIPT_GENERATION_ERROR);
        }

        if config_lines.len() < MINIMUM_CONFIGURATION_LINES {
            eprintln!("Error: Configuration file must contain at least 2 lines (title and one character)");
            return Err(SCRIPT_GENERATION_ERROR);
        }

        *title = config_lines[TITLE_CONFIGURATION_INDEX].clone();

        let mut line_index = FIRST_CHARACTER_CONFIGURATION_INDEX;

        while line_index < config_lines.len() {
            Play::add_config(&config_lines[line_index], config);
            line_index += CONFIGURATION_INDEX_STEP;
        }

        Ok(())

    }

    pub fn prepare(&mut self, config_filename: &String) -> Result<(), u8> {

        let mut config: PlayConfig = Vec::new();

        if let Err(_error_code) = Play::read_config(config_filename, &mut self.play_title, &mut config) {
            return Err(SCRIPT_GENERATION_ERROR);
        }

        if let Err(_error_code) = self.process_config(&config) {
            return Err(SCRIPT_GENERATION_ERROR);
        }

        Ok(())

    }

    pub fn recite(&mut self) {

        println!("{}", self.play_title);

        let mut current_character = String::new();
        let mut current_line = FIRST_CHARACTER_LINE;

        loop {

            let mut characters_speaking: HashSet<String> = HashSet::new();
            let mut characters_left_to_speak = false;

            for character in &mut self.play_characters {

                let mut character_has_spoken = false;

                loop {
                    match character.next_line() {
                        Some(line_number) if line_number == current_line => {
                            character.speak(&mut current_character);
                            characters_speaking.insert(current_character.clone());
                            if !character_has_spoken {
                                character_has_spoken = true;
                            } else if character_has_spoken && WHINGE_MODE.load(Ordering::SeqCst) {
                                println!("Warning: Character '{}' has more than {} lines at line number {}", current_character, MAXIMUM_LINES_PER_LINE_NUMBER, current_line);
                            }
                        },
                        Some(_) => characters_left_to_speak = true,
                        None => break
                    }
                }

            }

            if characters_speaking.len() > MAXIMUM_CHARACTERS_PER_LINE_NUMBER && WHINGE_MODE.load(Ordering::SeqCst) {
                println!("Warning: More than {} characters {:?} are speaking at line number {}", MAXIMUM_CHARACTERS_PER_LINE_NUMBER, characters_speaking, current_line);
            }

            if characters_speaking.is_empty() && WHINGE_MODE.load(Ordering::SeqCst) {
                println!("Warning: No characters are speaking at line number {}", current_line);
            }

            if !characters_left_to_speak {
                break;
            }

            current_line += CHARACTER_LINE_STEP;

        }

    }

}
