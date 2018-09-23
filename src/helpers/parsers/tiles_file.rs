use helpers::parsers::find_attribute;
use helpers::parsers::parser;
use sdl::TextureWrapper;
use std::collections::HashMap;
use super::xml::reader::XmlEvent;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum XmlReadingState {
    Root,
    InTileset,
    InTilesetImage,
}


pub fn parse(filename: &str, textures: &mut Vec<(String, String)>, texture_wrappers: &mut HashMap<String, TextureWrapper>) {
    let mut state = XmlReadingState::Root;
    let mut tile_height = 0;
    let mut tile_width = 0;
    let mut tile_count = 0;
//    let mut margin = 0;
    let mut spacing = 0;

    for e in parser(filename) {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                let local_name = name.local_name.to_ascii_lowercase();
                state = match (state, local_name.as_str()) {
                    (XmlReadingState::Root, "tileset") => {
                        tile_height = find_attribute(&attributes, "tileheight").expect("Missing tiles height");
                        tile_width = find_attribute(&attributes, "tilewidth").expect("Missing tiles width");
                        tile_count = find_attribute(&attributes, "tilecount").expect("Missing tilecount width");
                        spacing = find_attribute(&attributes, "spacing").expect("Missing spacing width");
//                        margin = find_attribute(&attributes, "margin").expect("Missing margin width");
                        XmlReadingState::InTileset
                    }
                    (XmlReadingState::InTileset, "image") => {
                        let source: String = find_attribute(&attributes, "source").expect("Missing source");
                        let key = String::from("tiles");
                        texture_wrappers.insert(
                            key.clone(),
                            TextureWrapper::new(key.clone(), tile_width, tile_height, spacing, tile_count),
                        );
                        textures.push((key, format!("assets/{}", source)));

                        XmlReadingState::InTilesetImage
                    }
                    _ => {
                        println!("unknown element: {:?} :: {:?}", local_name, state);
                        state
                    }
                }
            }
            Ok(XmlEvent::EndElement { name, .. }) => {
                let local_name = name.local_name.to_ascii_lowercase();
                state = match (state, local_name.as_str()) {
                    (XmlReadingState::InTileset, "tileset") => { XmlReadingState::Root }
                    (XmlReadingState::InTilesetImage, "image") => { XmlReadingState::InTileset }
                    _ => {
                        println!("unknown element: {:?} :: {:?}", local_name, state);
                        state
                    }
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
    use sdl::TextureWrapper;
    use std::collections::HashMap;

    #[test]
    fn test_parsing() {
        //given
        let mut textures = Vec::new();
        let mut texture_wrappers = HashMap::new();

        //when
        parsers::tiles_file::parse("assets/tiles.tsx", &mut textures, &mut texture_wrappers);

        //then
        assert_eq!(textures.len(), 1);
        assert_eq!(textures[0], (String::from("tiles"), String::from("assets/tiles.png")));

        assert_eq!(texture_wrappers.len(), 1);
        assert_eq!(texture_wrappers.get("tiles"), Some(&TextureWrapper::new(String::from("tiles"), 32, 32, 1, 3)));
    }
}
