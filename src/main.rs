extern crate base64;
extern crate cgmath;
extern crate core;

use game::Scene;
use helpers::parsers;
use std::thread;
use std::time::Duration;
use std::time::SystemTime;

mod game;
pub mod helpers;
mod sdl;

const FPS: u8 = 60;

pub fn main() {
    let delay: Duration = Duration::new(0, 1000000000 / FPS as u32);

    let engine = sdl::SDLEngine::new();

    let (canvas, texture_creator) = sdl::Renderer::init(&engine);
    let texture_manager = sdl::TextureManager::new(&texture_creator);

    let (player, game_objects, texture_wrappers) = parsers::map_file::parse();

    let scene = Scene::new(player, game_objects);

    let video = sdl::Renderer::new(canvas, texture_manager, texture_wrappers);
    let input_handler = sdl::InputHandler::new(&engine);

    let mut game = game::Engine::new(video, input_handler, scene);
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
        frame += 1;
    }
}
