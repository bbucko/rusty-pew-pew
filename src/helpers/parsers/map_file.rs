use base64;
use game::GameObject;
use game::Level;
use game::states;
use helpers::parsers::find_attribute;
use helpers::parsers::inflate;
use helpers::parsers::parser;
use helpers::parsers::xml::reader::XmlEvent;
use sdl::TextureWrapper;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum XmlReadingState {
    Root,
    InMap,
    InMapTileset,
    InMapLayer,
    InMapLayerData,
    InMapObjectgroup,
    InMapObjectgroupObject,
}


pub fn parse(filename: &str) -> (Vec<Option<GameObject>>, Level, HashMap<String, TextureWrapper>, (u8, u8, u8), String) {
    let mut state = XmlReadingState::Root;
    let mut properties: HashMap<String, String> = HashMap::new();

    let mut game_objects = Vec::new();
    let mut texture_wrappers = HashMap::new();
    let mut tiles = Vec::new();
    let mut width = 0;
    let mut height = 0;
    let mut color = (0, 0, 0);
    let mut tiles_filename: String = String::new();
    for e in parser(filename) {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                let local_name = name.local_name.to_ascii_lowercase();
                state = match (state, local_name.as_str()) {
                    (XmlReadingState::Root, "map") => {
                        height = find_attribute(&attributes, "height").expect("Missing tiles height");
                        width = find_attribute(&attributes, "width").expect("Missing tiles width");
                        let background_color: String = find_attribute(&attributes, "backgroundcolor").expect("Missing background color");

                        let r = u8::from_str_radix(&background_color[1..3], 16).unwrap();
                        let g = u8::from_str_radix(&background_color[3..5], 16).unwrap();
                        let b = u8::from_str_radix(&background_color[5..7], 16).unwrap();

                        color = (r, g, b);

                        XmlReadingState::InMap
                    }
                    (XmlReadingState::InMap, "layer") => XmlReadingState::InMapLayer,
                    (XmlReadingState::InMap, "tileset") => {
                        tiles_filename = find_attribute(&attributes, "source").expect("Missing tiles.tsx");
                        XmlReadingState::InMapTileset
                    }
                    (XmlReadingState::InMap, "objectgroup") => XmlReadingState::InMapObjectgroup,
                    (XmlReadingState::InMapObjectgroup, "object") => {
                        let available_keys = ["name", "type", "width", "height", "x", "y"].iter();
                        properties.extend(available_keys.map(|key| {
                            let value: String =
                                find_attribute(&attributes, key).unwrap_or_else(|| panic!("missing {:?}", key));
                            (key.to_string(), value.clone())
                        }));
                        XmlReadingState::InMapObjectgroupObject
                    }
                    (XmlReadingState::InMapObjectgroupObject, "properties") => XmlReadingState::InMapObjectgroupObject,
                    (XmlReadingState::InMapObjectgroupObject, "property") => {
                        let name: String = find_attribute(&attributes, "name").expect("missing name");
                        let value: String = find_attribute(&attributes, "value").expect("missing value");

                        properties.insert(name, value);
                        XmlReadingState::InMapObjectgroupObject
                    }
                    (XmlReadingState::InMapLayer, "data") => XmlReadingState::InMapLayerData,
                    _ => {
                        println!("unknown element: {:?} :: {:?}", local_name, state);
                        state
                    }
                };
            }
            Ok(XmlEvent::Characters(value)) => {
                if state == XmlReadingState::InMapLayerData {
                    let inflated = base64::decode(value.trim()).expect("Incorrectly encoded");
                    let deflated = inflate::inflate_bytes_zlib(&inflated).expect("Incorrectly compressed");

                    let every_fourth = deflated.iter()
                        .enumerate()
                        .filter(|(i, _)| i % 4 == 0 )
                        .map(|(_, value)| value)
                        .into_iter();

                    tiles.extend(every_fourth);
                }
            }
            Ok(XmlEvent::EndElement { name, .. }) => {
                let local_name = name.local_name.to_ascii_lowercase();
                state = match (state, local_name.as_str()) {
                    (XmlReadingState::InMap, "map") => XmlReadingState::Root,
                    (XmlReadingState::InMapLayer, "layer") => XmlReadingState::InMap,
                    (XmlReadingState::InMapTileset, "tileset") => XmlReadingState::InMap,
                    (XmlReadingState::InMapObjectgroup, "objectgroup") => XmlReadingState::InMap,
                    (XmlReadingState::InMapObjectgroupObject, "object") => {
                        let result = states::create_game_object(&properties).unwrap();
                        game_objects.push(Some(result));

                        let texture_id = properties.get("textureID").expect("Missing textureID").to_string();
                        let width = properties.get("width").expect("Missing width").parse().unwrap();
                        let height = properties.get("height").expect("Missing height").parse().unwrap();
                        let frames = properties.get("numFrames").map_or(1, |s| s.parse().unwrap());
                        let padding = properties
                            .get("padding")
                            .map_or(if frames == 1 { 0 } else { 1 }, |s| s.parse().unwrap());

                        texture_wrappers.insert(
                            texture_id.clone(),
                            TextureWrapper::new(texture_id, width, height, padding, frames),
                        );
                        properties.clear();
                        XmlReadingState::InMapObjectgroup
                    }
                    (XmlReadingState::InMapObjectgroupObject, "properties") => XmlReadingState::InMapObjectgroupObject,
                    (XmlReadingState::InMapObjectgroupObject, "property") => XmlReadingState::InMapObjectgroupObject,
                    (XmlReadingState::InMapLayerData, "data") => XmlReadingState::InMapLayer,
                    _ => {
                        println!("end of unknown element: {:?} :: {:?}", local_name, state);
                        state
                    }
                };
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }

    (game_objects, Level::new(width, height, tiles), texture_wrappers, color, tiles_filename)
}

#[cfg(test)]
mod tests {
    use game::Id;
    use helpers::parsers;

    #[test]
    fn test_parsing() {
        let (game_objects, level, texture_wrappers, color, tiles_filename) = parsers::map_file::parse("assets/map1.tmx");
        assert_eq!(game_objects.len(), 3);

        let ids: Vec<Id> = game_objects
            .into_iter()
            .filter(|s| s.is_some())
            .map(|maybe| maybe.unwrap())
            .map(|game_object| game_object.id)
            .collect();

        assert_eq!(ids[0], 1);
        assert_eq!(ids[1], 2);
        assert_eq!(ids[2], 3);

        assert_eq!(texture_wrappers.len(), 2);

        assert_eq!(level.tiles.len(), (level.width * level.height) as usize);
        assert_eq!(&level.tiles[0..20], &vec![3; 20][..]);
        assert_eq!(level.width, 20);
        assert_eq!(level.height, 60);

        assert_eq!(color, (2, 45, 155));

        assert_eq!(tiles_filename, "tiles.tsx".to_string());
    }
}
