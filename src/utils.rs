pub mod xml {

    use std::fs::File;
    use std::io::BufReader;
    use xml::attribute::OwnedAttribute;
    use xml::EventReader;

    pub fn parser(filename: &str) -> EventReader<BufReader<File>> {
        let file = File::open(filename).unwrap();
        let file = BufReader::new(file);

        EventReader::new(file)
    }

    pub fn find_attribute(attributes: &Vec<OwnedAttribute>, name: &str) -> String {
        for attr in attributes {
            if attr.name.local_name.to_ascii_lowercase() == name {
                return String::from(attr.value.clone());
            }
        }
        panic!("Unable to parse textures");
    }
}
