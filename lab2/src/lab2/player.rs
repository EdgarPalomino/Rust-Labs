use std::sync::atomic::Ordering;

use super::declarations::{SCRIPT_GENERATION_ERROR, WHINGE_MODE};

type PlayLines = Vec<(usize, String)>;

const FIRST_CHARACTER_LINE: usize = 0;
const CHARACTER_LINE_STEP: usize = 1;

pub struct Player {
    character_name: String,
    character_lines: PlayLines,
    current_line: usize
}

impl Player {

    pub fn new(name: &String) -> Player {
        Player {character_name: name.clone(), character_lines: Vec::new(), current_line: FIRST_CHARACTER_LINE}
    }

    fn add_script_line(&mut self, line: &String) {
        if !line.is_empty() {
            if let Some((first_token, rest_of_line)) = line.split_once(char::is_whitespace) {
                // Try to parse the first token as line number
                if let Ok(line_number) = first_token.parse::<usize>() {
                    self.character_lines.push((line_number, rest_of_line.trim().to_string()));
                } else if WHINGE_MODE.load(Ordering::SeqCst) {
                    println!("Warning: '{}' does not represent a valid line number", first_token);
                }
            }
        }
    }

    pub fn prepare(&mut self, part_file_name: &String) -> Result<(), u8> {

        let mut part_lines: Vec<String> = Vec::new();

        if let Err(_error_code) = super::script_gen::grab_trimmed_file_lines(part_file_name, &mut part_lines) {
            return Err(SCRIPT_GENERATION_ERROR);
        }

        for line in &part_lines {
            self.add_script_line(line)
        }

        self.character_lines.sort();

        Ok(())

    }

    pub fn speak(&mut self, current_character: &mut String) {

        if !(self.current_line < self.character_lines.len()) {
            return;
        }

        if self.character_name != *current_character {
            *current_character = self.character_name.clone();
            println!();
            println!("{}.", current_character)
        }

        let (_line_number, line_text) = &self.character_lines[self.current_line];

        println!("{}", line_text);

        self.current_line += CHARACTER_LINE_STEP;

    }

    pub fn next_line(&self) -> Option<usize> {

        if !(self.current_line < self.character_lines.len()) {
            return None;
        }

        let (line_number, _line_text) = &self.character_lines[self.current_line];

        Some(*line_number)

    }

}
