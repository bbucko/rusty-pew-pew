extern crate cgmath;

use game::player::Player;
use std::collections::LinkedList;
use std::fmt::Debug;

mod player;

pub type Event = u8;

pub type Pos = cgmath::Vector2<f32>;

pub type Transl = cgmath::Vector2<f32>;

pub trait Renderer {
    fn draw(&mut self);
}

pub trait InputHandler {
    fn handle(&mut self) -> Vec<Event>;
}

pub trait Position {}

pub trait GameObject: Debug {
    fn draw(&self, &Renderer);
    fn update(&mut self);
}

pub struct Game<R: Renderer, IH: InputHandler> {
    running: bool,
    video: R,
    input_handler: IH,
    player: Player,
    game_objects: LinkedList<Box<GameObject>>,
}

impl<T: Renderer, U: InputHandler> Game<T, U> {
    pub fn new(video: T, input_handler: U) -> Game<T, U> {
        let game_objects: LinkedList<Box<GameObject>> = LinkedList::new();
        let player = Player::new();
        Game {
            running: true,
            video,
            input_handler,
            player,
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
        &self.player.update();
        for mut game_object in &self.game_objects {
            println!("{:?}", game_object);
        }
        return;
    }

    pub fn render(&mut self) {
        self.video.draw();
    }
}
