use std::fs;
use dirs;
use std::io::{stdin, stdout, Write};
use crate::goto_info::GotoInfo;

pub fn get_chosen_index(info: &GotoInfo) -> usize {
    let mut choice: usize = 0;
    let mut should_continue = false;

    while !should_continue {
        info.print_choices();

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

pub fn get_goto_info() -> GotoInfo {
    let hd = match dirs::home_dir() {
        Some(dir) => format!("{}", dir.display()),
        None => panic!("Unable to get home directory")
    };
    let config_file = format!("{}/.goto", &hd);
    let output_file = format!("/tmp/goto.loc");

    let data = fs::read_to_string(&config_file)
        .expect("Unable to read goto file");

    let locations = match serde_json::from_str(&data) {
        Ok(locs) => locs,
        Err(_) => panic!("Unable to parse goto file to json")
    };

    GotoInfo{
        home_dir: hd,
        config_file,
        output_file,
        locations
    }
}

pub fn write_to_file(file: &str, data: &str) {
    match fs::write(file, data) {
        Ok(_) => (),
        Err(_) => panic!("Cannot write data new locations to config file")
    }
}