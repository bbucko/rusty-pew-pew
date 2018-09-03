use cgmath::Vector2;
use game::player::Player;
use helpers::parsers;
use sdl::TextureWrapper;
use std::collections::HashMap;
use std::fmt::Debug;

mod enemy;
pub mod factory;
pub mod player;

pub type Event = u8;
pub type Pos = Vector2<f32>;
pub type Transl = Vector2<f32>;

pub trait Renderer {
    fn update_wrappers(&mut self, wrappers: HashMap<String, TextureWrapper>);
    fn render(&mut self, game_objects: &[Box<GameObject>], player: &Player);
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

pub struct Engine<R: Renderer, IH: InputHandler> {
    running: bool,
    video: R,
    input_handler: IH,
    player: Player,
    _level: Level,
    game_objects: Vec<Box<GameObject>>,
}

impl<T: Renderer, U: InputHandler> Engine<T, U> {
    pub fn new(mut video: T, input_handler: U) -> Self {
        let _level = Level {};
        let (player, game_objects, texture_wrappers) = parsers::map_file::parse();

        video.update_wrappers(texture_wrappers);

        Engine {
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

    pub fn render(&mut self) {
        self.video.render(&self.game_objects, &self.player);
    }
}
