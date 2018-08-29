extern crate sdl2;

use game::Pos;
use game::Renderer;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::video::WindowContext;
use sdl::SDLVideo;
use sdl::TextureManager;
use sdl::TextureWrapper;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use xml::EventReader;
use xml::reader::XmlEvent;

impl<'a> Renderer for SDLVideo<'a> {
    fn prepare(&mut self) {
        self.canvas.clear();
    }

    fn draw(&mut self) {
        self.canvas.present();
    }

    fn draw_texture(&mut self, texture_id: &str, position: Pos) {
        let texture = self.texture_manager.load(texture_id)
            .expect("Error loading texture");

        let src_rect = None;
        let dst_rect = Rect::new(position.x as i32, position.y as i32, 300, 100);

        self.canvas
            .copy(&texture, src_rect, dst_rect)
            .expect("Problem copying texture");
    }

    fn draw_frame(&mut self, texture_id: &str, position: Pos, frame: u8) {
        //texture_wrapper should be always present
        let texture_wrapper = self.objects.entry(String::from(texture_id))
            .or_insert(TextureWrapper { texture_id: String::from(texture_id), width: 64, height: 64, padding: 1, frames: 3 });

        println!("Drawing frame {} out of {}", frame, texture_wrapper.frames);

        let texture = self.texture_manager.load(&texture_wrapper.texture_id)
            .expect("Error loading texture");

        let dst_rect = Rect::new(position.x as i32, position.y as i32, texture_wrapper.height, texture_wrapper.width);

        self.canvas
            .copy(&texture, texture_wrapper.src_rect(), dst_rect)
            .expect("Problem copying texture");
    }
}

impl<'a> SDLVideo<'a> {
    pub fn init(canvas: Canvas<Window>, texture_manager: TextureManager<'a, WindowContext>) -> SDLVideo {
        let objects = HashMap::new();

        load();

        SDLVideo { canvas, texture_manager, objects }
    }
}

fn load() {
    let file = File::open("assets/game.xml").unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut depth = 0;
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                println!("{}+{}", indent(depth), name);
                depth += 1;
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                println!("{}-{}", indent(depth), name);
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
}

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
        .fold(String::with_capacity(size * INDENT.len()), |r, s| r + s)
}

