use crate::goto_location::GotoLocation;
use crate::utils::{write_to_file, get_user_input};

#[derive(Debug)]
pub struct GotoInfo {
    pub home_dir: String,
    pub config_file: String,
    pub output_file: String,
    pub locations: Vec<GotoLocation>
}

impl GotoInfo {
    pub fn update_selected_location(&mut self, index: usize) {
        let selected_location = &mut self.locations[index];

        selected_location.increment_frequency();

        self.locations.sort_by(|a, b| a.frequency.cmp(&b.frequency));

        let output_data = match serde_json::to_string(&self.locations) {
            Ok(data) => data,
            Err(_) => panic!("Cannot serialize locations to json")
        };

        write_to_file(&self.config_file, &output_data);
    }

    pub fn update_output_file(&self, location_index: usize) {
        let location = &self.locations[location_index];
        let expanded_location = str::replace(&location.destination, "~", &self.home_dir);

        write_to_file(&self.output_file, &expanded_location);
    }

    pub fn print_choices(&self) {
        for (i, loc) in self.locations.iter().enumerate() {
            println!("{}) {}", i+1, loc.name);
        }
    }

    pub fn rm_location(&mut self, index: usize) {
        let _ = self.locations.remove(index);

        let output_data = self.locations_to_json();

        write_to_file(&self.config_file, &output_data);
    }

    pub fn add_location(&mut self, loc: GotoLocation) {
        self.locations.push(loc);

        let output_data = self.locations_to_json();

        write_to_file(&self.config_file, &output_data);
    }

    pub fn locations_to_json(&self) -> String {
        match serde_json::to_string(&self.locations) {
            Ok(data) => data,
            Err(_) => panic!("Cannot serialize locations to json")
        }
    }

    pub fn edit_location(&mut self, index: usize) {
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