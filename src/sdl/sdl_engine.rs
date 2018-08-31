use sdl::SDLEngine;
use sdl2::image::init as sdl2_image_init;
use sdl2::image::{INIT_JPG, INIT_PNG};
use sdl2::init as sdl2_init;

impl SDLEngine {
    pub fn new() -> SDLEngine {
        let sdl = sdl2_init().expect("Error initializing SDL2");
        let _sdl_image =
            sdl2_image_init(INIT_PNG | INIT_JPG).expect("Error initializing SDL2 Image");

        SDLEngine { context: sdl }
    }
}
