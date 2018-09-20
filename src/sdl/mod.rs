extern crate sdl2;

use self::sdl2::render::Canvas;
use self::sdl2::render::Texture;
use self::sdl2::render::TextureCreator;
use self::sdl2::video::Window;
use self::sdl2::video::WindowContext;
use self::sdl2::EventPump;
use self::sdl2::Sdl;
use sdl::resource_manager::ResourceManager;
use sdl::sdl2::image::init as sdl2_image_init;
use sdl::sdl2::image::{INIT_JPG, INIT_PNG};
use sdl::sdl2::init as sdl2_init;
use sdl::sdl2::TimerSubsystem;
use std::collections::HashMap;

mod resource_manager;
mod sdl_input_handler;
mod sdl_video;

pub type TextureManager<'l, T> = ResourceManager<'l, String, Texture<'l>, TextureCreator<T>>;

pub struct SDLEngine {
    pub context: Sdl
}

pub struct SDLRenderer<'a> {
    canvas: Canvas<Window>,
    texture_manager: TextureManager<'a, WindowContext>,
    texture_wrappers: HashMap<String, TextureWrapper>,
    timer: TimerSubsystem
}

pub struct SDLInputHandler {
    event_pump: EventPump,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TextureWrapper {
    texture_id: String,
    width: u32,
    height: u32,
    padding: u8,
    frames: u8,
}

impl SDLEngine {
    pub fn init() -> SDLEngine {
        let sdl = sdl2_init().expect("Error initializing SDL2");
        let _sdl_image = sdl2_image_init(INIT_PNG | INIT_JPG).expect("Error initializing SDL2 Image");

        SDLEngine { context: sdl }
    }
}
