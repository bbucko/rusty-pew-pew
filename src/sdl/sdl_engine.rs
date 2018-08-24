extern crate sdl2;

use sdl::SDLEngine;

impl SDLEngine {
    pub fn init() -> SDLEngine {
        SDLEngine {
            context: sdl2::init().unwrap()
        }
    }
}
