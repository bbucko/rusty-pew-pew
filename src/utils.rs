pub mod xml {

    use std::fs::File;
    use std::io::BufReader;
    use xml::attribute::OwnedAttribute;
    use xml::EventReader;
    use std::str::FromStr;

    pub fn parser(filename: &str) -> EventReader<BufReader<File>> {
        let file = File::open(filename).unwrap();
        let file = BufReader::new(file);

        EventReader::new(file)
    }

    pub fn find_attribute<T: FromStr>(attributes: &[OwnedAttribute], name: &str) -> Option<T> {
        for attr in attributes {
            if attr.name.local_name.to_ascii_lowercase() == name {
                let result= attr.value.parse();
                return match result {
                    Ok(value) => Some(value),
                    _ => None
                }
            }
        }
        None
    }
}
