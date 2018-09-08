use base64::decode;
use game::GameState;
use helpers::parsers::find_attribute;
use helpers::parsers::inflate::inflate_bytes_zlib;
use helpers::parsers::parser;
use helpers::parsers::xml::reader::XmlEvent;
use sdl::TextureWrapper;
use std::collections::HashMap;
use game::states;

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

pub fn parse(filename: &str) -> (GameState, HashMap<String, TextureWrapper>) {
    let mut state = XmlReadingState::Root;
    let mut properties: HashMap<String, String> = HashMap::new();

    let mut game_objects = Vec::new();
    let mut texture_wrappers = HashMap::new();
    let mut tiles = Vec::new();
    let mut width = 0;
    let mut height = 0;

    for e in parser(filename) {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                let local_name = name.local_name.to_ascii_lowercase();
                state = match (state, local_name.as_str()) {
                    (XmlReadingState::Root, "map") => {
                        height = find_attribute(&attributes, "height").expect("Missing tiles height");
                        width = find_attribute(&attributes, "width").expect("Missing tiles width");
                        XmlReadingState::InMap
                    }
                    (XmlReadingState::InMap, "layer") => XmlReadingState::InMapLayer,
                    (XmlReadingState::InMap, "tileset") => XmlReadingState::InMapTileset,
                    (XmlReadingState::InMap, "objectgroup") => XmlReadingState::InMapObjectgroup,
                    (XmlReadingState::InMapObjectgroup, "object") => {
                        let available_keys = ["name", "type", "width", "height", "x", "y"].iter();
                        properties.extend(available_keys.map(|key| {
                            let value: String = find_attribute(&attributes, key).expect(&format!("missing {:?}", key));
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
                    let inflated = decode(value.trim()).unwrap();
                    let deflated = inflate_bytes_zlib(&inflated);
                    tiles.extend(deflated.expect("Missing tiles").iter());
                    println!("Parsed layers");
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

    (GameState::new(game_objects, tiles, width, height), texture_wrappers)
}

#[cfg(test)]
mod tests {
    use game::Id;
    use helpers::parsers;

    #[test]
    fn test_parsing() {
        let (scene, texture_wrappers) = parsers::map_file::parse("assets/map1.tmx");
        assert_eq!(scene.game_objects.len(), 3);

        let ids: Vec<Id> = scene.game_objects.into_iter()
            .filter(|s| s.is_some())
            .map(|maybe| maybe.unwrap())
            .map(|game_object| game_object.id)
            .collect();

        assert_eq!(ids[0], 1);
        assert_eq!(ids[1], 2);
        assert_eq!(ids[2], 3);

        assert_eq!(texture_wrappers.len(), 2);
    }
}
