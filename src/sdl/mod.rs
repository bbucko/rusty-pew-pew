extern crate sdl2;

use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use sdl::resource_manager::ResourceManager;
use self::sdl2::EventPump;
use self::sdl2::render::Canvas;
use self::sdl2::video::Window;
use std::collections::HashMap;

mod sdl_engine;
mod sdl_input_handler;
mod sdl_video;
mod resource_manager;

pub type TextureManager<'l, T> = ResourceManager<'l, String, sdl2::render::Texture<'l>, TextureCreator<T>>;

pub struct SDLEngine {
    pub context: sdl2::Sdl,
}

pub struct SDLVideo<'a> {
    canvas: Canvas<Window>,
    texture_manager: TextureManager<'a, WindowContext>,
    objects: HashMap<String, TextureWrapper>,
}

pub struct SDLInputHandler {
    event_pump: EventPump,
}

pub struct TextureWrapper {
    texture_id: String,
    width: u32,
    height: u32,
    padding: u8,
    frames: u8,
}

impl TextureWrapper {
    pub fn src_rect(&self) -> Rect {
        Rect::new(self.padding as i32, self.padding as i32, self.width, self.height)
    }
}
