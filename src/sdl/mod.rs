use sdl::resource_manager::ResourceManager;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::video::Window;
use sdl2::video::WindowContext;
use sdl2::EventPump;
use sdl2::Sdl;
use std::collections::HashMap;

mod resource_manager;
mod sdl_engine;
mod sdl_input_handler;
mod sdl_video;

pub type TextureManager<'l, T> = ResourceManager<'l, String, Texture<'l>, TextureCreator<T>>;

pub struct SDLEngine {
    pub context: Sdl,
}

pub struct Renderer<'a> {
    canvas: Canvas<Window>,
    texture_manager: TextureManager<'a, WindowContext>,
    objects: HashMap<String, TextureWrapper>,
}

pub struct InputHandler {
    event_pump: EventPump,
}

pub struct TextureWrapper {
    texture_id: String,
    width: u32,
    height: u32,
    padding: u8,
    frames: u8,
}
