use game::Game;
use std::thread;
use std::time::Duration;
use std::time::SystemTime;

mod game;
mod sdl;

const FPS: u8 = 60;

pub fn main() {
    let delay: Duration = Duration::new(0, 1000000000 / FPS as u32);

    let engine = sdl::SDLEngine::init();
    let video = sdl::SDLVideo::init(&engine);
    let input_handler = sdl::SDLInputHandler::init(&engine);

    let mut game = Game::new(video, input_handler);

    while game.is_running() {
        let frame_start = SystemTime::now();

        game.handle_events();

        game.update();

        game.render();

        let duration = SystemTime::now().duration_since(frame_start).unwrap();
        if duration.le(&delay) {
            thread::sleep(delay - duration);
        }
    }
}
