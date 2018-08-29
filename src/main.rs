extern crate sdl2;
extern crate xml;

use game::Game;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::render::TextureCreator;
use sdl2::video::Window;
use sdl2::video::WindowContext;
use sdl::SDLEngine;
use sdl::TextureManager;
use std::thread;
use std::time::Duration;
use std::time::SystemTime;

mod game;
mod sdl;

const FPS: u8 = 60;

pub fn main() {
    let delay: Duration = Duration::new(0, 1000000000 / FPS as u32);

    let engine = sdl::SDLEngine::init();

    let (canvas, texture_creator) = create_canvas(&engine);
    let texture_manager = TextureManager::new(&texture_creator);

    let video = sdl::SDLVideo::init(canvas, texture_manager);
    let input_handler = sdl::SDLInputHandler::init(&engine);

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

fn create_canvas(engine: &SDLEngine) -> (Canvas<Window>, TextureCreator<WindowContext>) {
    let video_subsystem = engine.context.video().unwrap();
    let window = video_subsystem
        .window("rusty pew pew", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .expect("Error creating window");
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    let texture_creator = canvas.texture_creator();
    (canvas, texture_creator)
}
