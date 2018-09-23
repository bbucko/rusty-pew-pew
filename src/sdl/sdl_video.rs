use game::GameObject;
use game::Level;
use game::Position;
use game::Renderer;
use helpers::parsers;
use SCREEN_SIZE;
use sdl::sdl2::pixels::Color;
use sdl::sdl2::rect::Rect;
use sdl::sdl2::render::Canvas;
use sdl::sdl2::render::TextureCreator;
use sdl::sdl2::TimerSubsystem;
use sdl::sdl2::video::Window;
use sdl::sdl2::video::WindowContext;
use sdl::SDLEngine;
use sdl::SDLRenderer;
use sdl::TextureManager;
use sdl::TextureWrapper;
use std::collections::HashMap;

impl<'a> Renderer for SDLRenderer<'a> {
    fn render(&mut self, game_objects: &mut [Option<GameObject>], level: &Level) {
        self.canvas.clear();

        for game_object in game_objects {
            if let Some(game_object) = game_object {
                game_object.draw(self, level);
            }
        }

        self.canvas.present();
    }

    fn draw_texture(&mut self, texture_id: &str, position: Position, level: &Level) {
        let texture_wrapper = self.texture_wrappers.get(texture_id).expect("Missing texture wrapper");
        let texture = self.texture_manager.load(texture_id).expect("Error loading texture");

        let src_rect = texture_wrapper.src_rect(0);

        let position_on_screen = position - level.position;

        let dst_rect = Rect::new(
            position_on_screen.x as i32,
            position_on_screen.y as i32,
            texture_wrapper.width,
            texture_wrapper.height,
        );

        self.canvas
            .copy(&texture, src_rect, dst_rect)
            .expect("Problem copying texture");
    }

    fn draw_frame(&mut self, texture_id: &str, position: Position, level: &Level) {
        let ticks = self.timer.ticks();

        let texture_wrapper = self.texture_wrappers.get(texture_id).expect("Missing texture wrapper");
        let texture = self.texture_manager.load(texture_id).expect("Error loading texture");
        let frame = (ticks / 100) % u32::from(texture_wrapper.frames);

        let src_rect = texture_wrapper.src_rect(frame);

        let position_on_screen = position - level.position;

        let dst_rect = Rect::new(
            position_on_screen.x as i32,
            position_on_screen.y as i32,
            texture_wrapper.width,
            texture_wrapper.height,
        );

        self.canvas
            .copy(&texture, src_rect, dst_rect)
            .expect("Problem copying texture");
    }
}

impl<'a> SDLRenderer<'a> {
    pub fn init(engine: &SDLEngine, color: (u8, u8, u8)) -> (Canvas<Window>, TextureCreator<WindowContext>, TimerSubsystem) {
        let timer = engine.context.timer().unwrap();
        let video_subsystem = engine.context.video().unwrap();
        let (screen_width, screen_height) = SCREEN_SIZE;
        let window = video_subsystem
            .window("rusty pew pew", screen_width, screen_height)
            .position_centered()
            .opengl()
            .build()
            .expect("Error creating window");

        let mut canvas = window.into_canvas().accelerated().build().unwrap();
        canvas.set_draw_color(Color::RGB(color.0, color.1, color.2));

        let texture_creator = canvas.texture_creator();
        (canvas, texture_creator, timer)
    }

    pub fn new(canvas: Canvas<Window>,
               mut texture_manager: TextureManager<'a, WindowContext>,
               mut texture_wrappers: HashMap<String, TextureWrapper>,
               tiles_filename: &str,
               timer: TimerSubsystem) -> Self {
        Self::load_textures(&mut texture_manager, &mut texture_wrappers);
        Self::load_tiles(tiles_filename, &mut texture_manager, &mut texture_wrappers);

        Self {
            canvas,
            texture_manager,
            texture_wrappers,
            timer,
        }
    }

    fn load_textures(texture_manager: &mut TextureManager<'a, WindowContext>,
                     texture_wrappers: &mut HashMap<String, TextureWrapper>, ) {
        let mut textures = Vec::new();
        parsers::game_file::parse("assets/game.xml", &mut textures, texture_wrappers);

        for element in textures {
            let (key, filename) = element;
            texture_manager
                .preload(&key, &filename)
                .expect("Error preloading texture");
        }
    }

    fn load_tiles(filename: &str,
                  texture_manager: &mut TextureManager<'a, WindowContext>,
                  texture_wrappers: &mut HashMap<String, TextureWrapper>, ) {
        let mut textures = Vec::new();
        parsers::tiles_file::parse(&format!("assets/{}", filename), &mut textures, texture_wrappers);

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
    pub fn src_rect(&self, frame: u32) -> Rect {
        let padding = u32::from(self.padding);
        let width = self.width;
        let height = self.height;
        let x = (frame * (width + padding) + padding) as i32;
        let y = padding as i32;
        Rect::new(x, y, width, height)
    }
}
