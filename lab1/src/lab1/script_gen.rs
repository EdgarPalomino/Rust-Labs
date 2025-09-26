use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::sync::atomic::Ordering;

use super::declarations::{Play, WHINGE_MODE, SCRIPT_GENERATION_ERROR};

pub type PlayConfig = Vec<(String, String)>; // (character_name, part_file_name)

pub const TITLE_LINE_INDEX: usize = 0;        
pub const FIRST_CHARACTER_LINE_INDEX: usize = 1; 
pub const CHARACTER_NAME_INDEX: usize = 0;
pub const PART_FILE_NAME_INDEX: usize = 1;
pub const CONFIG_LINE_TOKEN_COUNT: usize = 2;

pub fn add_script_line(play: &mut Play, line: &String, part_name: &String) {
    if line.len() > 0 {
        if let Some((first_token, rest_of_line)) = line.split_once(char::is_whitespace) {
            let trimmed_rest = rest_of_line.trim();
            
            // Try to parse the first token as line number
            match first_token.parse::<usize>() {
                Ok(line_number) => {
                    play.push((line_number, part_name.clone(), trimmed_rest.to_string()));
                }
                Err(_error_code) => {
                    if WHINGE_MODE.load(Ordering::SeqCst) {
                        println!("Warning: '{}' does not represent a valid line number", first_token);
                    }
                }
            }
        }
    }
}

pub fn grab_trimmed_file_lines(filename: &String, lines: &mut Vec<String>) -> Result<(), u8> {
    // The core function used for extracting data from files
    // Used for both reading the config file line by line and reading the parts file line by line
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(error_code) => {
            eprintln!("Error: Failed to open file '{}': {}", filename, error_code);
            return Err(SCRIPT_GENERATION_ERROR);
        }
    };
    
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    
    loop {
        line.clear();
        
        let bytes_read = match reader.read_line(&mut line) {
            Ok(bytes) => bytes,
            Err(error_code) => {
                eprintln!("Error: Failed to read line from file '{}': {}", filename, error_code);
                return Err(SCRIPT_GENERATION_ERROR);
            }
        };
        
        if bytes_read == 0 {
            return Ok(());
        }

        lines.push(line.trim().to_string());
    }
}

pub fn process_config(play: &mut Play, config: &PlayConfig) -> Result<(), u8> {
    for config_entry in config {
        match config_entry {
            (part_name, part_file_name) => {
                let mut part_lines: Vec<String> = Vec::new();
                
                if let Err(_error_code) = grab_trimmed_file_lines(part_file_name, &mut part_lines) {
                    return Err(SCRIPT_GENERATION_ERROR);
                }
                
                for line in &part_lines {
                    add_script_line(play, line, part_name);
                }
            }
        }
    }
    
    Ok(())
}

pub fn add_config(config_line: &String, config: &mut PlayConfig) {
    let tokens: Vec<&str> = config_line.split_whitespace().collect();
    
    if tokens.len() != CONFIG_LINE_TOKEN_COUNT {
        if WHINGE_MODE.load(Ordering::SeqCst) {
            if tokens.len() < CONFIG_LINE_TOKEN_COUNT {
                println!("Warning: Configuration line has too few tokens (expected {}, got {}): '{}'", 
                         CONFIG_LINE_TOKEN_COUNT, tokens.len(), config_line);
            } else {
                println!("Warning: Configuration line has too many tokens (expected {}, got {}): '{}'", 
                         CONFIG_LINE_TOKEN_COUNT, tokens.len(), config_line);
            }
        }
    }
    
    if tokens.len() >= CONFIG_LINE_TOKEN_COUNT {
        config.push((
            tokens[CHARACTER_NAME_INDEX].to_string(),
            tokens[PART_FILE_NAME_INDEX].to_string()
        ));
    }
}

pub fn read_config(config_filename: &String, title: &mut String, config: &mut PlayConfig) -> Result<(), u8> {
    let mut config_lines: Vec<String> = Vec::new();
    
    if let Err(_error_code) = grab_trimmed_file_lines(config_filename, &mut config_lines) {
        return Err(SCRIPT_GENERATION_ERROR);
    }
    
    if config_lines.len() < 2 {
        eprintln!("Error: Configuration file must contain at least 2 lines (title and one character)");
        return Err(SCRIPT_GENERATION_ERROR);
    }
    
    *title = config_lines[TITLE_LINE_INDEX].clone();
    
    for line_index in FIRST_CHARACTER_LINE_INDEX..config_lines.len() {
        add_config(&config_lines[line_index], config);
    }
    
    Ok(())
}

pub fn script_gen(config_filename: &String, title: &mut String, play: &mut Play) -> Result<(), u8> {
    let mut config: PlayConfig = Vec::new();
    
    if let Err(_error_code) = read_config(config_filename, title, &mut config) {
        return Err(SCRIPT_GENERATION_ERROR);
    }
    
    if let Err(_error_code) = process_config(play, &config) {
        return Err(SCRIPT_GENERATION_ERROR);
    }
    
    Ok(())
}
    