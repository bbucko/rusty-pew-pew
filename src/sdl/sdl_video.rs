use game::GameObject;
use game::Position;
use game::Renderer;
use helpers::parsers;
use sdl::sdl2::pixels::Color;
use sdl::sdl2::rect::Rect;
use sdl::sdl2::render::Canvas;
use sdl::sdl2::render::TextureCreator;
use sdl::sdl2::video::Window;
use sdl::sdl2::video::WindowContext;
use sdl::SDLEngine;
use sdl::SDLRenderer as SDLRenderer;
use sdl::TextureManager;
use sdl::TextureWrapper;
use std::collections::HashMap;

impl<'a> Renderer for SDLRenderer<'a> {
    fn render(&mut self, game_objects: &mut Vec<Option<GameObject>>) {
        self.canvas.clear();

        for game_object in game_objects {
            if let Some(game_object) = game_object {
                game_object.draw(self);
            }
        }

        self.canvas.present();
    }

    fn draw_texture(&mut self, texture_id: &str, position: Position) {
        self.draw_frame(texture_id, position, 0);
    }

    fn draw_frame(&mut self, texture_id: &str, position: Position, frame: u8) {
        let texture_wrapper = self.texture_wrappers.get(texture_id).expect("Missing texture wrapper");

        let texture = self.texture_manager.load(texture_id).expect("Error loading texture");

        let src_rect = texture_wrapper.src_rect(frame);
        let dst_rect = Rect::new(
            position.x as i32,
            position.y as i32,
            texture_wrapper.width,
            texture_wrapper.height,
        );

        self.canvas
            .copy(&texture, src_rect, dst_rect)
            .expect("Problem copying texture");
    }
}

impl<'a> SDLRenderer<'a> {
    pub fn init(engine: &SDLEngine) -> (Canvas<Window>, TextureCreator<WindowContext>) {
        let video_subsystem = engine.context.video().unwrap();

        let window = video_subsystem
            .window("rusty pew pew", 800, 600)
            .position_centered()
            .opengl()
            .build()
            .expect("Error creating window");

        let mut canvas = window.into_canvas().present_vsync().build().unwrap();
        canvas.set_draw_color(Color::RGB(0, 0, 0));

        let texture_creator = canvas.texture_creator();
        (canvas, texture_creator)
    }

    pub fn new(canvas: Canvas<Window>,
               mut texture_manager: TextureManager<'a, WindowContext>,
               texture_wrappers: HashMap<String, TextureWrapper>) -> Self {
        Self::load_textures(&mut texture_manager);

        Self {
            canvas,
            texture_manager,
            texture_wrappers,
        }
    }

    fn load_textures(texture_manager: &mut TextureManager<'a, WindowContext>) {
        let mut textures = Vec::new();
        parsers::game_file::parse(&mut textures);

        for element in textures {
            let (key, filename) = element;
            texture_manager
                .preload(&key, &filename)
                .expect("Error preloading texture");
        }
    }
}

impl TextureWrapper {
    pub fn new(texture_id: String, width: u32, height: u32, padding: u8, frames: u8) -> TextureWrapper {
        TextureWrapper {
            texture_id,
            width,
            height,
            padding,
            frames,
        }
    }
    pub fn src_rect(&self, _frame: u8) -> Rect {
        let padding = self.padding as u32;

        let width = self.width;
        let height = self.height;
        let x = padding as i32;
        let y = padding as i32;
        Rect::new(x, y, width, height)
    }
}
