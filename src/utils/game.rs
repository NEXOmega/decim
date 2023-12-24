extern crate serde;
extern crate serde_json;
use clap::Parser;

use serde_derive::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Serialize, Deserialize, Parser, Tabled, Clone, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Game {
    #[arg(short, long)]
    pub name: String,
    #[arg(short, long)]
    pub description: String,
    #[arg(short, long, default_value_t= String::from("0.1.0"))]
    pub version: String,
    #[arg(short, long)]
    #[tabled(display_with = "display_vec_str")]
    pub tags: Vec<String>,
    #[arg(short, long, default_value_t = String::from(""))]
    pub save_location: String,
    #[arg(short, long, default_value_t = String::from(""))]
    pub executable: String,
    #[arg(short, long)]
    #[tabled(display_with = "display_vec_str")]
    pub version_history: Vec<String>,
}

impl Game {
    pub fn new(name: String, description: String) -> Game {
        Game {
            name,
            description,
            tags: Vec::new(),
            version: String::from(""),
            save_location: String::from(""),
            executable: String::from(""),
            version_history: Vec::new(),
        }
    }

    pub fn add_tag(&mut self, tag: String) {
        self.tags.push(tag);
    }

    pub fn remove_tag(&mut self, tag: String) {
        let index = self.tags.iter().position(|x| *x == tag).unwrap();
        self.tags.remove(index);
    }

    pub fn edit_version(&mut self, version: String) {
        if self.version != String::from("") {
            self.version_history.push(self.version.clone());
        }
        self.version = version;
    }

    pub fn edit_save_location(&mut self, save_location: String) {
        self.save_location = save_location;
    }

    pub fn edit_executable(&mut self, executable: String) {
        self.executable = executable;
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn get_tags(&self) -> Vec<String> {
        self.tags.clone()
    }

    pub fn get_version(&self) -> String {
        self.version.clone()
    }

    pub fn get_save_location(&self) -> String {
        self.save_location.clone()
    }

    pub fn get_executable(&self) -> String {
        self.executable.clone()
    }
}

fn display_vec_str(tags: &Vec<String>) -> String {
    let mut tags = tags.clone();
    tags.sort();
    tags.join(", ")
}