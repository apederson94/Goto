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
    let mut info = get_goto_info();

    let selected_index = match info.locations.iter().position(|loc| loc.abbreviation == dest) {
        Some(i) => i,
        None => panic!("Unable to find a location matching the destination {}", dest)
    };

    info.update_selected_location(selected_index);
    info.update_output_file(selected_index);
}

fn interactive_path() {
    println!("No shortcut provided - interactive mode")
}

fn get_goto_info() -> GotoInfo {
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

fn write_to_file(file: &str, data: &str) {
    match fs::write(file, data) {
        Ok(_) => (),
        Err(_) => panic!("Cannot write data new locations to config file")
    }
}

#[derive(Debug)]
struct GotoInfo {
    home_dir: String,
    config_file: String,
    output_file: String,
    locations: Vec<GotoLocation>
}

impl GotoInfo {
    fn update_selected_location(&mut self, index: usize) {
        let selected_location = &mut self.locations[index];

        selected_location.increment_frequency();

        let output_data = match serde_json::to_string(&self.locations) {
            Ok(data) => data,
            Err(_) => panic!("Cannot serialize locations to json")
        };

        write_to_file(&self.config_file, &output_data);
    }

    fn update_output_file(&self, location_index: usize) {
        let location: &GotoLocation = &self.locations[location_index];

        write_to_file(&self.output_file, &location.destination);
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct GotoLocation {
    name: String,
    abbreviation: String,
    destination: String,
    frequency: i64
}

impl GotoLocation {
    fn increment_frequency(&mut self) {
        self.frequency += 1
    }
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
