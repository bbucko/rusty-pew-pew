extern crate sdl2;

use game::Renderer;
use sdl::SDLEngine;
use sdl::SDLVideo;
use self::sdl2::pixels::Color;

impl Renderer for SDLVideo {
    fn draw(&mut self) {
        self.canvas.clear();
        self.canvas.present();
    }
}

impl SDLVideo {
    pub fn init(sdl: &SDLEngine) -> SDLVideo {
        let video_subsystem = sdl.context.video().unwrap();

        let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        SDLVideo { canvas }
    }
}
