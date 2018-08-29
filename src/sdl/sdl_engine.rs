extern crate sdl2;

use self::sdl2::image::{INIT_JPG, INIT_PNG};
use sdl::SDLEngine;

impl SDLEngine {
    pub fn init() -> SDLEngine {
        let sdl = sdl2::init().unwrap();
        let _sdl_image = sdl2::image::init(INIT_PNG | INIT_JPG).unwrap();

        SDLEngine { context: sdl }
    }
}
