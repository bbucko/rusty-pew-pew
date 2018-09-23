use helpers::parsers::find_attribute;
use helpers::parsers::parser;
use helpers::parsers::xml::reader::XmlEvent;
use sdl::TextureWrapper;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum XmlReadingState {
    Root,
    InPlayTextures,
    InPlay,
}

pub fn parse(filename: &str, textures: &mut Vec<(String, String)>, texture_wrappers: &mut HashMap<String, TextureWrapper>) {
    let mut state = XmlReadingState::Root;

    let parser = parser(filename);

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

                        let width: Option<String> = find_attribute(&attributes, "width");
                        let height: Option<String> = find_attribute(&attributes, "height");

                        if let (Some(width), Some(height)) = (width, height) {
                            let padding = find_attribute(&attributes, "padding").unwrap_or(0);
                            let frames = find_attribute(&attributes, "frames").unwrap_or(0);
                            let width: u32 = width.parse().unwrap();
                            let height: u32 = height.parse().unwrap();

                            texture_wrappers.insert(
                                key.clone(),
                                TextureWrapper::new(key.clone(), width, height, padding, frames),
                            );
                        }

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
    use std::collections::HashMap;

    #[test]
    fn test_parsing() {
        //given
        let mut textures = Vec::new();
        let mut texture_wrappers = HashMap::new();

        //when
        parsers::game_file::parse("assets/game.xml", &mut textures, &mut texture_wrappers);

        //then
        assert_eq!(textures.len(), 3);
        assert!(textures.contains(&(String::from("plane"), String::from("assets/plane.png"))));
        assert!(textures.contains(&(String::from("whitePlane"), String::from("assets/whitePlane.png"))));
        assert!(textures.contains(&(String::from("bullet"), String::from("assets/bullet.png"))));
    }
}
