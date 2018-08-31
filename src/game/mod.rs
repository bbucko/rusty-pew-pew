use cgmath::Vector2;
use game::enemy::Enemy;
use game::player::Player;
use std::fmt::Debug;
use utils::xml::parser;
use xml::reader::XmlEvent;

mod enemy;
mod player;

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
    level: Level,
    game_objects: Vec<Box<GameObject>>,
}

impl<T: Renderer, U: InputHandler> Game<T, U> {
    pub fn new(video: T, input_handler: U) -> Self {
        let player = Player::new(Pos::new(10.0, 10.0));

        let mut game_objects: Vec<Box<GameObject>> = Vec::new();
        game_objects.push(Box::new(Enemy::new()));

        let level = Level {};

        Self::parse_map_file();

        //load map1.tmx
        //load tiles.tsx

        Game {
            running: true,
            video,
            input_handler,
            player,
            level,
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
                self.player.up();
            }
        }
    }

    pub fn update(&mut self) {
        self.player.update();

        for game_object in self.game_objects.iter_mut() {
            game_object.update();
        }
    }

    pub fn render(&mut self) {
        self.video.prepare();

        self.player.draw(&mut self.video);

        for game_object in self.game_objects.iter_mut() {
            game_object.draw(&mut self.video);
        }

        self.video.draw();
    }

    fn parse_map_file() {
        let parser = parser("assets/map1.tmx");

        for e in parser {
            match e {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    println!("{} :: {:?}", name, attributes);
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    println!("{}", name);
                }
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
                _ => {}
            }
        }
    }
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
