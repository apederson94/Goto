use std::fs;
use dirs;
use std::io::{stdin, stdout, Write};
use std::env;
use crate::config::Config;

pub fn get_chosen_index(config: &Config) -> usize {
    let mut choice: usize = 0;
    let mut should_continue = false;

    while !should_continue {
        config.print_choices();

        match get_user_input("Enter choice: ").parse::<usize>() {
            Ok(num) => {
                should_continue = true;
                choice = num - 1;
            },
            Err(_) => println!("Unable to parse input into a number. Please try again")
        };
    }

    return choice;
}

pub fn get_user_input(message: &str) -> String {
    print!("{}", message);

    let _ = stdout().flush();
    let mut input = String::new();
    stdin().read_line(&mut input).expect("No value entered");

    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }

    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }

    return input;
}

pub fn get_home_dir() -> String {
    match dirs::home_dir() {
        Some(dir) => format!("{}", dir.display()),
        None => panic!("Unable to get home directory")
    }
}

pub fn get_current_dir() -> String {
    match env::current_dir() {
        Ok(dir) => {
            let long_dir = format!("{}", dir.display());
            str::replace(&long_dir, &get_home_dir(), "~")
        },
        Err(_) => panic!("Unable to get current directory")
    }
}

pub fn get_config_path() -> String {
    format!("{}/.goto", get_home_dir())
}

pub fn get_config() -> Config {
    let config_file = get_config_path();

    let data = fs::read_to_string(&config_file)
        .expect("Unable to read goto file");

    match serde_json::from_str(&data) {
        Ok(config) => config,
        Err(_) => panic!("Unable to parse goto file to json")
    }
}

pub fn write_to_file(file: &str, data: &str) {
    match fs::write(file, data) {
        Ok(_) => (),
        Err(e) => panic!("Cannot write data to config file: {}", e)
    }
}