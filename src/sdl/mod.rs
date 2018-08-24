extern crate sdl2;

use self::sdl2::EventPump;
use self::sdl2::render::Canvas;
use self::sdl2::video::Window;

mod sdl_engine;
mod sdl_video;
mod sdl_input_handler;

pub struct SDLEngine {
    context: sdl2::Sdl
}

pub struct SDLVideo {
    canvas: Canvas<Window>
}

pub struct SDLInputHandler {
    event_pump: EventPump
}
