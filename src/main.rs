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
        Some(dest) => shortuct_path(dest),
        None => interactive_path()
    }
}

fn shortuct_path(dest: String) {
    let info = get_goto_info();
    println!("Shortcut name: {}", dest)
}

fn interactive_path() {
    println!("No shortcut provided - interactive mode")
}

fn get_goto_info() {
    let hd_path = match dirs::home_dir() {
        Some(dir) => dir,
        None => panic!("Unable to get home directory")
    };
    let hd = format!("{}", hd_path.display());
    let config_file = format!("{}/.goto", &hd);
    let output_file = format!("/tmp/goto.loc");

    let data = fs::read_to_string(config_file)
        .expect("Unable to read goto file");

    let locations: Vec<GotoLocation> = match serde_json::from_str(&data) {
        Ok(locs) => locs,
        Err(_) => panic!("Unable to parse goto file to json")
    };

    println!("{:?}", locations)
}

struct GotoInfo {
    home_dir: String,
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
