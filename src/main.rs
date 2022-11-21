use clap::Parser;
use std::fs;
use dirs;
use serde::{Serialize, Deserialize};
use std::io::{stdin, stdout, Write};

fn main() {
    let args: CliArgs = CliArgs::parse();

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

fn rm_path() {
    let mut info = get_goto_info();

    let choice = get_chosen_index(&info);

    info.rm_location(choice);
}

fn add_path() {
    let mut info = get_goto_info();

    let name = get_user_input("Name: ");
    let abbreviation = get_user_input("Abbreviation: ");
    let destination = get_user_input("Destination: ");

    let loc = GotoLocation {
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

fn get_chosen_index(info: &GotoInfo) -> usize {
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

fn get_user_input(message: &str) -> String {
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

        self.locations.sort_by(|a, b| a.frequency.cmp(&b.frequency));

        let output_data = match serde_json::to_string(&self.locations) {
            Ok(data) => data,
            Err(_) => panic!("Cannot serialize locations to json")
        };

        write_to_file(&self.config_file, &output_data);
    }

    fn update_output_file(&self, location_index: usize) {
        let location: &GotoLocation = &self.locations[location_index];
        let expanded_location = str::replace(&location.destination, "~", &self.home_dir);

        write_to_file(&self.output_file, &expanded_location);
    }

    fn print_choices(&self) {
        for (i, loc) in self.locations.iter().enumerate() {
            println!("{}) {}", i+1, loc.name);
        }
    }

    fn rm_location(&mut self, index: usize) {
        let _ = self.locations.remove(index);

        let output_data = self.locations_to_json();

        write_to_file(&self.config_file, &output_data);
    }

    fn add_location(&mut self, loc: GotoLocation) {
        self.locations.push(loc);

        let output_data = self.locations_to_json();

        write_to_file(&self.config_file, &output_data);
    }

    fn locations_to_json(&self) -> String {
        match serde_json::to_string(&self.locations) {
            Ok(data) => data,
            Err(_) => panic!("Cannot serialize locations to json")
        }
    }

    fn edit_location(&mut self, index: usize) {
        let loc = &mut self.locations[index];

        let message = format!("Enter name[{}]: ", loc.name);
        let new_name = get_user_input(&message);

        let message = format!("Enter abbreviation[{}]: ", loc.abbreviation);
        let new_abbrev = get_user_input(&message);

        let message = format!("Enter location[{}]: ", loc.destination);
        let new_dest = get_user_input(&message);

        if !new_name.is_empty() {
            loc.name = new_name;
        }

        if !new_abbrev.is_empty() {
            loc.abbreviation = new_abbrev;
        }

        if !new_dest.is_empty() {
            loc.destination = new_dest;
        }

        let output_data = self.locations_to_json();

        write_to_file(&self.config_file, &output_data);
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
