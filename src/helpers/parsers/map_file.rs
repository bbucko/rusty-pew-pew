use base64::decode;
use game::factory;
use game::player::Player;
use game::GameObject;
use game::Pos;
use helpers::parsers::find_attribute;
use helpers::parsers::inflate::inflate_bytes_zlib;
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

pub fn parse() -> (Player, Vec<Box<GameObject>>, HashMap<String, TextureWrapper>) {
    let mut state = XmlReadingState::Root;
    let mut properties: HashMap<String, String> = HashMap::new();

    let mut player = None;
    let mut game_objects: Vec<Box<GameObject>> = Vec::new();
    let mut texture_wrappers: HashMap<String, TextureWrapper> = HashMap::new();

    for e in parser("assets/map1.tmx") {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                let local_name = name.local_name.to_ascii_lowercase();
                state = match (state, local_name.as_str()) {
                    (XmlReadingState::Root, "map") => XmlReadingState::InMap,
                    (XmlReadingState::InMap, "layer") => XmlReadingState::InMapLayer,
                    (XmlReadingState::InMap, "tileset") => XmlReadingState::InMapTileset,
                    (XmlReadingState::InMap, "objectgroup") => XmlReadingState::InMapObjectgroup,
                    (XmlReadingState::InMapObjectgroup, "object") => {
                        properties.insert(
                            "name".to_string(),
                            find_attribute(&attributes, "name").expect("missing name"),
                        );
                        properties.insert(
                            "type".to_string(),
                            find_attribute(&attributes, "type").expect("missing type"),
                        );

                        properties.insert(
                            "width".to_string(),
                            find_attribute(&attributes, "width").expect("missing width"),
                        );
                        properties.insert(
                            "height".to_string(),
                            find_attribute(&attributes, "height").expect("missing height"),
                        );

                        properties.insert(
                            "x".to_string(),
                            find_attribute(&attributes, "x").expect("missing X"),
                        );
                        properties.insert(
                            "y".to_string(),
                            find_attribute(&attributes, "y").expect("missing Y"),
                        );
                        XmlReadingState::InMapObjectgroupObject
                    }
                    (XmlReadingState::InMapObjectgroupObject, "properties") => {
                        XmlReadingState::InMapObjectgroupObject
                    }
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
                    let _deflated = inflate_bytes_zlib(&inflated);
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
                        {
                            let object_type = properties.get("type").expect("Missing type");
                            if object_type == "Player" {
                                let x = properties.get("x").map_or(0, |s| s.parse().unwrap());
                                let y = properties.get("y").map_or(0, |s| s.parse().unwrap());
                                println!("{:?} :: {:?}", x, y);
                                player = Some(Player::new(Pos::new(x as f32, y as f32)));
                            } else {
                                game_objects.push(factory::create(&properties));
                            }
                        }

                        let texture_id = properties
                            .get("textureID")
                            .expect("Missing textureID")
                            .to_string();
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
                    (XmlReadingState::InMapObjectgroupObject, "properties") => {
                        XmlReadingState::InMapObjectgroupObject
                    }
                    (XmlReadingState::InMapObjectgroupObject, "property") => {
                        XmlReadingState::InMapObjectgroupObject
                    }
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
    (
        player.expect("Player was not created"),
        game_objects,
        texture_wrappers,
    )
}

#[cfg(test)]
mod tests {
    use helpers::parsers;

    #[test]
    fn test_parsing() {
        let (player, game_objects, texture_wrappers) = parsers::map_file::parse();
    }
}
