extern crate base64;
extern crate cgmath;
extern crate core;
#[macro_use]
extern crate lazy_static;

use helpers::parsers;
use std::thread;
use std::time::Duration;
use std::time::SystemTime;

mod game;
mod helpers;
mod sdl;

const FPS: u8 = 60;

lazy_static! {
    static ref DELAY: Duration = Duration::new(0, 1_000_000_000 / FPS as u32);
}

pub fn main() {
    println!("Starting up");

    let sdl_context = sdl::SDLEngine::init();

    let (game_state, texture_wrappers) = parsers::map_file::parse("assets/map1.tmx");

    let input_handler = sdl::SDLInputHandler::new(&sdl_context);

    let (canvas, texture_creator) = sdl::SDLRenderer::init(&sdl_context);
    let texture_manager = sdl::TextureManager::new(&texture_creator);
    let renderer = sdl::SDLRenderer::new(canvas, texture_manager, texture_wrappers);

    let mut engine = game::Engine::new(game_state, renderer, input_handler);

    while engine.is_running {
        let frame_start = SystemTime::now();

        engine.handle_input();
        engine.update();
        engine.draw();

        frame_sync_wait(frame_start);
    }

    println!("Shutting down. Goodbye!");
}

fn frame_sync_wait(frame_start: SystemTime) {
    let now = SystemTime::now();
    let duration = now.duration_since(frame_start).unwrap();
    if duration.le(&DELAY) {
        thread::sleep(*DELAY - duration)
    }
}