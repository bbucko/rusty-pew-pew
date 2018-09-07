use game::InputHandler;
use game::InputState;
use sdl::SDLInputHandler as SDLInputHandler;
use sdl::sdl2::event::Event;
use sdl::sdl2::keyboard::Keycode;
use sdl::SDLEngine;

impl InputHandler for SDLInputHandler {
    fn capture(&mut self) -> Option<InputState> {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    return Some(1);
                }
                Event::KeyDown { keycode: Some(key), .. } => {
                    println!("key pressed: {:?}", key.name());
                    return Some(2);
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
        None
    }
}

impl SDLInputHandler {
    pub fn new(sdl: &SDLEngine) -> Self {
        let event_pump = sdl.context.event_pump().unwrap();
        Self { event_pump }
    }
}
