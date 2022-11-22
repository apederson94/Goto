mod utils;
mod args;
mod config;
mod shortcut;

use crate::args::Args;
use crate::config::Config;
use crate::shortcut::Shortcut;

use clap::Parser;
use std::path::Path;
use std::fs::File;
use std::process;


fn main() {
    let args: Args = Args::parse();

    let did_init = init_if_necessary();

    if !did_init {
        let mut config = utils::get_config();

        if args.add_flag {
            add_path(&mut config);
        } else if args.edit_flag {
            edit_path(&mut config);
        } else if args.remove_flag {
            rm_path(&mut config);
        } else {
            if !config.shortcuts.is_empty() {
                match args.dest {
                    Some(dest) => shortcut_path(dest, &mut config),
                    _none => interactive_path(&mut config)
                }

                process::exit(0);
            } else {
                println!("Oops! Looks like you don't have any shortcuts yet. Try adding one with the -a flag");
            }
        }
    } else {
        println!("Done! Try adding a new shortcut by running again with the -a flag to add a new shortcut");
    }

    process::abort();
}

fn init_if_necessary() -> bool {
    let config_path = utils::get_config_path();
    let config_exists = Path::new(&config_path).exists();

    if !config_exists {
        println!("It looks like you haven't used Goto before. Let's get you setup!");

        let input_path = utils::get_user_input("Where do you want to store the last used location?[default: /tmp/goto.loc]: ");

        let output_path = if input_path.is_empty() {
            "/tmp/goto.loc".to_string()
        } else {
            input_path
        };

        match File::create(&config_path) {
            Ok(_) => println!("You're all setup now! Your config file lives at ~/.goto should you ever want to view it."),
            Err(_) => panic!("Unable to create the config file for you :(")
        };

        let config = Config {
            output_path,
            shortcuts: vec!()
        };

        let data = match serde_json::to_string(&config) {
            Ok(d) => d,
            Err(_) => panic!("Unable to pares new config file to json string")
        };

        utils::write_to_file(&config_path, &data);
    }

    !config_exists
}

fn add_path(config: &mut Config) {
    let name = utils::get_user_input("Name: ");
    let abbreviation = utils::get_user_input("Abbreviation: ");
    let dest_input = utils::get_user_input("Destination[default: current directory]: ");

    let destination = if dest_input.is_empty() {
        utils::get_current_dir()
    } else {
        dest_input
    };

    let shortcut = Shortcut {
        name,
        abbreviation,
        destination,
        frequency: 0
    };

    config.add_shortcut(shortcut);
}

fn edit_path(config: &mut Config) {
    let choice = utils::get_chosen_index(&config);

    config.edit_shortcut(choice);
}

fn rm_path(config: &mut Config) {
    let choice = utils::get_chosen_index(&config);

    config.rm_shortcut(choice);
}


fn shortcut_path(dest: String, config: &mut Config) {
    let selected_index = match config.shortcuts.iter().position(|loc| loc.abbreviation == dest) {
        Some(i) => i,
        None => panic!("Unable to find a shortcut matching the destination {}", dest)
    };

    config.update_output_file(selected_index);
    config.update_selected_shortcut(selected_index);
}

fn interactive_path(config: &mut Config) {
    let choice = utils::get_chosen_index(&config);

    config.update_output_file(choice);
    config.update_selected_shortcut(choice);
}
