use helpers::parsers::element_is;
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
                if state == XmlReadingState::Root {
                    if element_is(&name, "play") {
                        state = XmlReadingState::InPlay
                    }
                } else if state == XmlReadingState::InPlay {
                    if element_is(&name, "textures") {
                        state = XmlReadingState::InPlayTextures;
                    }
                } else if state == XmlReadingState::InPlayTextures {
                    if element_is(&name, "texture") {
                        let key: String = find_attribute(&attributes, "id").unwrap();
                        let filename = find_attribute(&attributes, "filename").unwrap();

                        textures.push((key.clone(), filename));
                    }
                }
            }
            Ok(XmlEvent::EndElement { name, .. }) => {
                if state == XmlReadingState::InPlayTextures && element_is(&name, "textures") {
                    state = XmlReadingState::InPlay;
                }

                if element_is(&name, "play") {
                    state = XmlReadingState::Root;
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
