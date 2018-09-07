extern crate inflate;
extern crate xml;

use helpers::parsers::xml::attribute::OwnedAttribute;
use helpers::parsers::xml::EventReader;
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;

pub mod game_file;
pub mod map_file;

pub fn parser(filename: &str) -> EventReader<BufReader<File>> {
    let file = File::open(filename).unwrap();
    let file = BufReader::new(file);

    EventReader::new(file)
}

pub fn find_attribute<T: FromStr>(attributes: &[OwnedAttribute], name: &str) -> Option<T> {
    for attr in attributes {
        if attr.name.local_name.to_ascii_lowercase() == name.to_ascii_lowercase() {
            let result = attr.value.parse();
            return match result {
                Ok(value) => Some(value),
                _ => None,
            };
        }
    }
    None
}
