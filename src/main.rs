use clap::Parser;
use std::fs;
use dirs;
use serde::{Serialize, Deserialize};

fn main() {
    let args: CliArgs = CliArgs::parse();

    if args.add_flag {
        println!("Add flag present");
    }

    if args.edit_flag {
        println!("Edit flag present");
    }

    if args.remove_flag {
        println!("Remove flag present")
    }

    match args.dest {
        Some(dest) => shortcut_path(dest),
        None => interactive_path()
    }
}

fn shortcut_path(dest: String) {
    let info = get_goto_info();

    let selected_index = match info.locations.iter().position(|loc| loc.abbreviation == dest) {
        Some(i) => i,
        None => panic!("Unable to find a location matching the destination {}", dest)
    };

    println!("Shortcut name: {}", dest);
    // println!("Selected location: {:?}", selected_location);

    update_config_file(info, selected_index);


}

fn interactive_path() {
    println!("No shortcut provided - interactive mode")
}

fn get_goto_info() -> GotoInfo {
    let hd_path = match dirs::home_dir() {
        Some(dir) => dir,
        None => panic!("Unable to get home directory")
    };
    let hd = format!("{}", hd_path.display());
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

fn update_config_file(info: GotoInfo, selected_index: usize) {
    let selected_location = &info.locations[selected_index];

    let new_location = GotoLocation {
        name: (*selected_location.name).to_string(),
        abbreviation: (*selected_location.abbreviation).to_string(),
        destination: (*selected_location.destination).to_string(),
        frequency: selected_location.frequency + 1
    };

    let mut new_locations = info.locations;

    let _ = std::mem::replace(&mut new_locations[selected_index], new_location);

    let output_data = match serde_json::to_string(&new_locations) {
        Ok(data) => data,
        Err(_) => panic!("Cannot serialize locations to json")
    };

    match fs::write(info.config_file, output_data) {
        Ok(_) => (),
        Err(_) => panic!("Cannot write data new locations to config file")
    }
}

struct GotoInfo {
    home_dir: String,
    config_file: String,
    output_file: String,
    locations: Vec<GotoLocation>
}

#[derive(Serialize, Deserialize, Debug)]
struct GotoLocation {
    name: String,
    abbreviation: String,
    destination: String,
    frequency: i64
}

#[derive(Parser)]
struct CliArgs {
    dest: Option<String>,
    #[arg(short = 'a')]
    add_flag: bool,
    #[arg(short = 'e')]
    edit_flag: bool,
    #[arg(short = 'r')]
    remove_flag: bool,
}
