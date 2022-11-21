mod utils;
mod cli_args;
mod goto_info;
mod goto_location;

use crate::cli_args::CliArgs;
use crate::utils::{get_user_input, get_goto_info, get_chosen_index};

use clap::Parser;


fn main() {
    let args: CliArgs = cli_args::CliArgs::parse();

    if args.add_flag {
        add_path();
    } else if args.edit_flag {
        edit_path();
    } else if args.remove_flag {
        rm_path();
    } else {
        match args.dest {
            Some(dest) => shortcut_path(dest),
            _none => interactive_path()
        }
    }
}

fn add_path() {
    let mut info = get_goto_info();

    let name = get_user_input("Name: ");
    let abbreviation = get_user_input("Abbreviation: ");
    let destination = get_user_input("Destination: ");

    let loc = goto_location::GotoLocation {
        name,
        abbreviation,
        destination,
        frequency: 0
    };

    info.add_location(loc);
}

fn edit_path() {
    let mut info = get_goto_info();

    let choice = get_chosen_index(&info);

    info.edit_location(choice);
}

fn rm_path() {
    let mut info = get_goto_info();

    let choice = get_chosen_index(&info);

    info.rm_location(choice);
}


fn shortcut_path(dest: String) {
    let mut info = get_goto_info();

    let selected_index = match info.locations.iter().position(|loc| loc.abbreviation == dest) {
        Some(i) => i,
        None => panic!("Unable to find a location matching the destination {}", dest)
    };

    info.update_output_file(selected_index);
    info.update_selected_location(selected_index);
}

fn interactive_path() {
    let mut info = get_goto_info();
    
    let choice = get_chosen_index(&info);

    info.update_output_file(choice);
    info.update_selected_location(choice);
}
