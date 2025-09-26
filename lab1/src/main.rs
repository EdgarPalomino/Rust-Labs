use std::env;
use std::sync::atomic::Ordering;

pub mod lab1;

use lab1::declarations::{Play, MIN_ARGS, MAX_ARGS, PROGRAM_NAME_INDEX, CONFIG_FILE_INDEX, 
                         VERBOSE_FLAG_INDEX, BAD_COMMAND_LINE_ERROR, WHINGE_MODE, SCRIPT_GENERATION_ERROR};

fn usage(program_name: &String) {
    println!("usage: {} <configuration_file_name> [whinge]", program_name);
}

fn parse_args(config_file: &mut String) -> Result<(), u8> {
    let mut args: Vec<String> = Vec::new();
    for arg in env::args() {
        args.push(arg);
    }

    if args.len() < MIN_ARGS || args.len() > MAX_ARGS {
        usage(&args[PROGRAM_NAME_INDEX]);
        return Err(BAD_COMMAND_LINE_ERROR);
    }

    if args.len() == MAX_ARGS && args[VERBOSE_FLAG_INDEX] != "whinge" {
        usage(&args[PROGRAM_NAME_INDEX]);
        return Err(BAD_COMMAND_LINE_ERROR);
    }

    *config_file = args[CONFIG_FILE_INDEX].clone();

    if args.len() == MAX_ARGS && args[VERBOSE_FLAG_INDEX] == "whinge" {
        WHINGE_MODE.store(true, Ordering::SeqCst);
    }
    
    Ok(())
}

fn recite(title: &String, play: &Play) {
    println!("{}", title);
    
    let mut current_character = String::new();
    
    for line_tuple in play {
        match line_tuple {
            (_line_number, character_name, line_text) => {
                // character_name is a borrowed reference created using type matching
                // this also works current_character != *character_name
                // I spent so long on this :(
                if &current_character != character_name {
                    current_character = character_name.clone();
                    println!();
                    println!("{}.", character_name);
                }
                
                println!("{}", line_text);
            }
        }
    }
}
    
fn main() -> Result<(), u8> {
    let mut config_file_name = String::new();

    if let Err(error_code) = parse_args(&mut config_file_name) {
        return Err(error_code);
    }

    let mut play_title = String::new();
    let mut play: Play = Vec::new();

    if let Err(_error_code) = lab1::script_gen::script_gen(&config_file_name, &mut play_title, &mut play) {
        return Err(SCRIPT_GENERATION_ERROR);
    }

    play.sort();
    
    recite(&play_title, &play);

    Ok(())
}
