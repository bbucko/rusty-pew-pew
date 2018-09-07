use helpers::parsers::find_attribute;
use helpers::parsers::parser;
use helpers::parsers::xml::reader::XmlEvent;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum XmlReadingState {
    Root,
    InPlayTextures,
    InPlay,
}

pub fn parse(textures: &mut Vec<(String, String)>) {
    let mut state = XmlReadingState::Root;

    let parser = parser("assets/game.xml");

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                let local_name = name.local_name.to_ascii_lowercase();
                state = match (state, local_name.as_str()) {
                    (XmlReadingState::Root, "play") => XmlReadingState::InPlay,
                    (XmlReadingState::InPlay, "textures") => XmlReadingState::InPlayTextures,
                    (XmlReadingState::InPlayTextures, "texture") => {
                        let key: String = find_attribute(&attributes, "id").unwrap();
                        let filename = find_attribute(&attributes, "filename").unwrap();

                        textures.push((key.clone(), filename));
                        XmlReadingState::InPlayTextures
                    }
                    _ => state,
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                let local_name = name.local_name.to_ascii_lowercase();
                state = match (state, local_name.as_str()) {
                    (XmlReadingState::InPlay, "play") => XmlReadingState::Root,
                    (XmlReadingState::InPlayTextures, "textures") => XmlReadingState::InPlay,
                    (XmlReadingState::InPlayTextures, "texture") => XmlReadingState::InPlayTextures,
                    _ => state,
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use helpers::parsers;

    #[test]
    fn test_parsing() {
        //given
        let mut textures = Vec::new();

        //when
        parsers::game_file::parse(&mut textures);

        //then
        assert_eq!(textures.len(), 3);
        assert!(textures.contains(&(String::from("plane"), String::from("assets/plane.png"))));
        assert!(textures.contains(&(String::from("whitePlane"), String::from("assets/whitePlane.png"))));
        assert!(textures.contains(&(String::from("bullet"), String::from("assets/bullet.png"))));
    }
}
