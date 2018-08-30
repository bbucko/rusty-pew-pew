extern crate sdl2;
extern crate xml;

use game::Game;
use sdl::TextureManager;
use std::thread;
use std::time::Duration;
use std::time::SystemTime;

mod game;
mod sdl;

const FPS: u8 = 60;

pub fn main() {
    let delay: Duration = Duration::new(0, 1000000000 / FPS as u32);

    let engine = sdl::SDLEngine::new();

    let (canvas, texture_creator) = sdl::Renderer::init(&engine);
    let texture_manager = TextureManager::new(&texture_creator);

    let video = sdl::Renderer::new(canvas, texture_manager);
    let input_handler = sdl::InputHandler::new(&engine);

    let mut game = Game::new(video, input_handler);
    let mut frame = 0;
    while game.is_running() {
        println!("tick: {:?}", frame);
        let frame_start = SystemTime::now();

        game.handle_events();

        game.update();

        game.render();

        let duration = SystemTime::now().duration_since(frame_start).unwrap();
        if duration.le(&delay) {
            thread::sleep(delay - duration);
        }
        frame = frame + 1;
    }
}
