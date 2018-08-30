use game::Event as GameEvent;
use game::InputHandler;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl::InputHandler as SDLInputHandler;
use sdl::SDLEngine;

impl InputHandler for SDLInputHandler {
    fn handle(&mut self) -> Vec<GameEvent> {
        let mut vec = Vec::new();

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown {
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
    pub fn new(sdl: &SDLEngine) -> Self {
        let event_pump = sdl.context.event_pump().unwrap();
        Self { event_pump }
    }
}
