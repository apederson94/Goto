use crate::shortcut::Shortcut;
use serde::{Serialize, Deserialize};
use super::utils;


#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub output_path: String,
    pub shortcuts: Vec<Shortcut>
}

impl Config {
    pub fn update_selected_shortcut(&mut self, index: usize) {
        let selected_location = &mut self.shortcuts[index];

        selected_location.increment_frequency();

        self.shortcuts.sort_by(|a, b| a.frequency.cmp(&b.frequency));

        let output_data = self.to_json();

        utils::write_to_file(&utils::get_config_path(), &output_data);
    }

    pub fn rm_shortcut(&mut self, index: usize) {
        let _ = self.shortcuts.remove(index);

        let output_data = self.to_json();

        utils::write_to_file(&utils::get_config_path(), &output_data);
    }

    pub fn add_shortcut(&mut self, shortcut: Shortcut) {
        self.shortcuts.push(shortcut);

        let output_data = self.to_json();

        utils::write_to_file(&utils::get_config_path(), &output_data);
    }

    pub fn edit_shortcut(&mut self, index: usize) {
        let loc = &mut self.shortcuts[index];

        let message = format!("Enter name[{}]: ", loc.name);
        let new_name = utils::get_user_input(&message);

        let message = format!("Enter abbreviation[{}]: ", loc.abbreviation);
        let new_abbrev = utils::get_user_input(&message);

        let message = format!("Enter location[{}]: ", loc.destination);
        let new_dest = utils::get_user_input(&message);

        if !new_name.is_empty() {
            loc.name = new_name;
        }

        if !new_abbrev.is_empty() {
            loc.abbreviation = new_abbrev;
        }

        if !new_dest.is_empty() {
            loc.destination = new_dest;
        }

        let output_data = self.to_json();

        utils::write_to_file(&utils::get_config_path(), &output_data);
    }

    pub fn update_output_file(&self, location_index: usize) {
        let location = &self.shortcuts[location_index];
        let expanded_location = str::replace(&location.destination, "~", &utils::get_home_dir());

        utils::write_to_file(&self.output_path, &expanded_location);
    }

    pub fn print_choices(&self) {
        for (i, loc) in self.shortcuts.iter().enumerate() {
            println!("{}) {}", i+1, loc.name);
        }
    }

    pub fn to_json(&self) -> String {
        match serde_json::to_string(&self) {
            Ok(data) => data,
            Err(_) => panic!("Cannot serialize shortcuts to json")
        }
    }
}