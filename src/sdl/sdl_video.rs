use game::Pos;
use game::Renderer;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::TextureCreator;
use sdl2::video::Window;
use sdl2::video::WindowContext;
use sdl::Renderer as SDLRenderer;
use sdl::SDLEngine;
use sdl::TextureManager;
use sdl::TextureWrapper;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use xml::attribute::OwnedAttribute;
use xml::EventReader;
use xml::reader::XmlEvent;

impl<'a> Renderer for SDLRenderer<'a> {
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
        //texture_wrapper should be always present. remove when map parsing is implemented
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


    pub fn new(canvas: Canvas<Window>, mut texture_manager: TextureManager<'a, WindowContext>) -> Self {
        let objects = HashMap::new();

        Self::load_textures(&mut texture_manager);

        Self { canvas, texture_manager, objects }
    }

    fn load_textures(texture_manager: &mut TextureManager<'a, WindowContext>) {
        let mut textures = Vec::new();
        Self::parse_file(&mut textures);

        for element in textures {
            let (key, filename) = element;
            texture_manager.preload(&key, &filename)
                .expect("Error preloading texture");
        }
    }

    fn find_attribute(attributes: &Vec<OwnedAttribute>, name: &str) -> String {
        for attr in attributes {
            if attr.name.local_name.to_ascii_lowercase() == name {
                return String::from(attr.value.clone());
            }
        }
        panic!("Unable to parse textures");
    }

    fn parse_file(textures: &mut Vec<(String, String)>) {
        let mut parsing_textures = false;
        let mut parsing_game = false;

        let file = File::open("assets/game.xml").unwrap();
        let file = BufReader::new(file);

        let parser = EventReader::new(file);

        for e in parser {
            match e {
                Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                    if name.local_name.to_ascii_lowercase() == "textures" {
                        parsing_textures = true;
                    } else if name.local_name.to_ascii_lowercase() == "play" {
                        parsing_game = true;
                    } else if name.local_name.to_ascii_lowercase() == "texture" && parsing_textures && parsing_game {
                        let mut key = Self::find_attribute(&attributes, "id");
                        let mut filename = Self::find_attribute(&attributes, "filename");

                        textures.push((key, filename));
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    if name.local_name.to_ascii_lowercase() == "textures" {
                        parsing_textures = false;
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
                _ => {}
            }
        }
    }
}

impl TextureWrapper {
    pub fn src_rect(&self) -> Rect {
        Rect::new(self.padding as i32, self.padding as i32, self.width, self.height)
    }
}


#[cfg(test)]
mod tests {
    use sdl::Renderer;

    #[test]
    fn test_parsing_xml() {
        //given
        let mut textures = Vec::new();

        //when
        Renderer::parse_file(&mut textures);

        //then
        assert_eq!(textures.len(), 3);
        assert!(textures.contains(&(String::from("plane"), String::from("assets/plane.png"))));
        assert!(textures.contains(&(String::from("whitePlane"), String::from("assets/whitePlane.png"))));
        assert!(textures.contains(&(String::from("bullet"), String::from("assets/bullet.png"))));
    }
}