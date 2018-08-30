extern crate sdl2;

use self::sdl2::event::Event;
use self::sdl2::keyboard::Keycode;
use game;
use game::InputHandler;
use sdl::SDLEngine;
use sdl::SDLInputHandler;

impl InputHandler for SDLInputHandler {
    fn handle(&mut self) -> Vec<game::Event> {
        let mut vec = Vec::new();

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    vec.push(1);
                }
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    println!("key pressed: {:?}", key);
                    vec.push(2);
                }
                Event::MouseMotion { .. }
                | Event::MouseWheel { .. }
                | Event::MouseButtonDown { .. }
                | Event::MouseButtonUp { .. } => {
                    //ignoring mouse
                }
                _ => {
                    println!("other: {:?}", event);
                }
            }
        }
        vec
    }
}

impl SDLInputHandler {
    pub fn new(sdl: &SDLEngine) -> SDLInputHandler {
        let event_pump = sdl.context.event_pump().unwrap();
        SDLInputHandler { event_pump }
    }
}
