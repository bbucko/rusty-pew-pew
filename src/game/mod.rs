use base64::decode;
use cgmath::Vector2;
use game::player::Player;
use inflate::inflate_bytes_zlib;
use std::collections::HashMap;
use std::fmt::Debug;
use utils::xml;
use utils::xml::parser;
use xml::reader::XmlEvent;

mod enemy;
mod player;
mod factory;

pub type Event = u8;
pub type Pos = Vector2<f32>;
pub type Transl = Vector2<f32>;

pub trait Renderer {
    fn prepare(&mut self);
    fn draw(&mut self);
    fn draw_texture(&mut self, texture_id: &str, position: Pos);
    fn draw_frame(&mut self, texture_id: &str, position: Pos, frame: u8);
}

pub trait InputHandler {
    fn handle(&mut self) -> Vec<Event>;
}

pub trait GameObject: Debug {
    fn draw(&self, renderer: &mut Renderer);

    fn update(&mut self);
}

struct Level {}

pub struct Game<R: Renderer, IH: InputHandler> {
    running: bool,
    video: R,
    input_handler: IH,
    player: Player,
    _level: Level,
    game_objects: Vec<Box<GameObject>>,
}

impl<T: Renderer, U: InputHandler> Game<T, U> {
    pub fn new(video: T, input_handler: U) -> Self {
        let _level = Level {};

        let (player, game_objects) = Self::parse_map_file();

        //load map1.tmx
        //load tiles.tsx

        Game {
            running: true,
            video,
            input_handler,
            player,
            _level,
            game_objects,
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn handle_events(&mut self) {
        let events = self.input_handler.handle();

        for event in events {
            if event == 1 {
                self.running = false;
            } else {
                self.player.down();
            }
        }
    }

    pub fn update(&mut self) {
        self.player.update();

        for game_object in &mut self.game_objects {
            game_object.update();
        }
    }

    fn parse_map_file() -> (Player, Vec<Box<GameObject>>) {
        let mut state = XmlReadingState::Off;
        let mut properties: HashMap<String, String> = HashMap::new();

        let mut player = None;
        let mut game_objects: Vec<Box<GameObject>> = Vec::new();

        for e in parser("assets/map1.tmx") {
            match e {
                Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                    let local_name = name.local_name.to_ascii_lowercase();
                    state = match (state, local_name.as_str()) {
                        (XmlReadingState::Off, "map") => XmlReadingState::InMap,
                        (XmlReadingState::InMap, "layer") => XmlReadingState::InMapLayer,
                        (XmlReadingState::InMap, "tileset") => XmlReadingState::InMapTileset,
                        (XmlReadingState::InMap, "objectgroup") => XmlReadingState::InMapObjectgroup,
                        (XmlReadingState::InMapObjectgroup, "object") => {
                            properties.insert("name".to_string(), xml::find_attribute(&attributes, "name").expect("missing name"));
                            properties.insert("type".to_string(), xml::find_attribute(&attributes, "type").expect("missing type"));
                            properties.insert("x".to_string(), xml::find_attribute(&attributes, "x").expect("missing X"));
                            properties.insert("y".to_string(), xml::find_attribute(&attributes, "y").expect("missing Y"));
                            XmlReadingState::InMapObjectgroupObject
                        }
                        (XmlReadingState::InMapObjectgroupObject, "properties") => {
                            XmlReadingState::InMapObjectgroupObject
                        }
                        (XmlReadingState::InMapObjectgroupObject, "property") => {
                            let name: String = xml::find_attribute(&attributes, "name").expect("missing name");
                            let value: String = xml::find_attribute(&attributes, "value").expect("missing value");

                            properties.insert(name, value);
                            XmlReadingState::InMapObjectgroupObject
                        }
                        (XmlReadingState::InMapLayer, "data") => XmlReadingState::InMapLayerData,
                        _ => {
                            println!("unknown: {:?} :: {:?}", local_name, state);
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
                        (XmlReadingState::InMap, "map") => XmlReadingState::Off,
                        (XmlReadingState::InMapLayer, "layer") => XmlReadingState::InMap,
                        (XmlReadingState::InMapTileset, "tileset") => XmlReadingState::InMap,
                        (XmlReadingState::InMapObjectgroup, "objectgroup") => XmlReadingState::InMap,
                        (XmlReadingState::InMapObjectgroupObject, "object") => {
                            println!("ready object: {:?}", properties);

                            if properties.get("type").expect("Missing type") == "Player" {
                                player = Some(Player::new(Pos::new(0.0, 0.0)));
                            } else {
                                game_objects.push(factory::create(&properties));
                            }

                            properties.clear();
                            XmlReadingState::InMapObjectgroup
                        }
                        (XmlReadingState::InMapObjectgroupObject, "properties") => XmlReadingState::InMapObjectgroupObject,
                        (XmlReadingState::InMapObjectgroupObject, "property") => XmlReadingState::InMapObjectgroupObject,
                        (XmlReadingState::InMapLayerData, "data") => XmlReadingState::InMapLayer,
                        _ => {
                            println!("end of unknown: {:?} :: {:?}", local_name, state);
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
        (player.expect("Player was not created"), game_objects)
    }

    pub fn render(&mut self) {
        self.video.prepare();

        self.player.draw(&mut self.video);

        for game_object in &mut self.game_objects {
            game_object.draw(&mut self.video);
        }

        self.video.draw();
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum XmlReadingState {
    Off,
    InMap,
    InMapTileset,
    InMapLayer,
    InMapLayerData,
    InMapObjectgroup,
    InMapObjectgroupObject,
}

#[cfg(test)]
mod tests {
    use game::Game;
    use sdl::InputHandler;
    use sdl::Renderer;

    type MyGame = Game<Renderer<'static>, InputHandler>;

    #[test]
    fn test_parsing() {
        MyGame::parse_map_file();
    }
}
