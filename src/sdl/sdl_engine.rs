use sdl2::image::{INIT_JPG, INIT_PNG};
use sdl2::image::init as sdl2_image_init;
use sdl2::init as sdl2_init;
use sdl::SDLEngine;

impl SDLEngine {
    pub fn new() -> SDLEngine {
        let sdl = sdl2_init().unwrap();
        let _sdl_image = sdl2_image_init(INIT_PNG | INIT_JPG).unwrap();

        SDLEngine { context: sdl }
    }
}
