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
use utils::xml;
use xml::reader::XmlEvent;

impl<'a> Renderer for SDLRenderer<'a> {
    fn prepare(&mut self) {
        self.canvas.clear();
    }

    fn draw(&mut self) {
        self.canvas.present();
    }

    fn draw_texture(&mut self, texture_id: &str, position: Pos) {
        self.draw_frame(texture_id, position, 0);
    }

    fn draw_frame(&mut self, texture_id: &str, position: Pos, frame: u8) {
        let texture_wrapper = self
            .objects
            .get(texture_id)
            .expect("Missing texture wrapper");

        println!("Drawing frame {} out of {}", frame, texture_wrapper.frames);

        let texture = self
            .texture_manager
            .load(texture_id)
            .expect("Error loading texture");

        let src_rect = texture_wrapper.src_rect();
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

    pub fn new(canvas: Canvas<Window>, mut texture_manager: TextureManager<'a, WindowContext>) -> Self {
        let mut objects = HashMap::new();

        Self::load_textures(&mut texture_manager, &mut objects);

        Self {
            canvas,
            texture_manager,
            objects,
        }
    }

    fn load_textures(texture_manager: &mut TextureManager<'a, WindowContext>, objects: &mut HashMap<String, TextureWrapper>) {
        let mut textures = Vec::new();
        Self::parse_game_file(&mut textures, objects);

        for element in textures {
            let (key, filename) = element;
            texture_manager
                .preload(&key, &filename)
                .expect("Error preloading texture");
        }
    }

    fn parse_game_file(textures: &mut Vec<(String, String)>, objects: &mut HashMap<String, TextureWrapper>) {
        let mut parsing_textures = false;
        let mut parsing_play_state = false;

        let parser = xml::parser("assets/game.xml");

        for e in parser {
            match e {
                Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                    if name.local_name.to_ascii_lowercase() == "textures" {
                        parsing_textures = true;
                    } else if name.local_name.to_ascii_lowercase() == "play" {
                        parsing_play_state = true;
                    } else if name.local_name.to_ascii_lowercase() == "texture" && parsing_textures && parsing_play_state {
                        let key: String = xml::find_attribute(&attributes, "id").unwrap();
                        let filename = xml::find_attribute(&attributes, "filename").unwrap();
                        let height = xml::find_attribute(&attributes, "height").expect("Missing height");
                        let width = xml::find_attribute(&attributes, "width").expect("Missing width");
                        let padding = xml::find_attribute(&attributes, "padding").unwrap_or(0);
                        let frames = xml::find_attribute(&attributes, "frames").unwrap_or(1);

                        textures.push((key.clone(), filename));
                        objects.insert(key.clone(), TextureWrapper { width, height, padding, frames });
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    if name.local_name.to_ascii_lowercase() == "textures" {
                        parsing_textures = false;
                    } else if name.local_name.to_ascii_lowercase() == "play" {
                        parsing_play_state = false;
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
    use sdl::TextureWrapper;
    use std::collections::HashMap;

    #[test]
    fn test_parsing_xml() {
        //given
        let mut textures = Vec::new();
        let mut objects = HashMap::new();

        //when
        Renderer::parse_game_file(&mut textures, &mut objects);

        //then
        assert_eq!(textures.len(), 3);
        assert!(textures.contains(&(String::from("plane"), String::from("assets/plane.png"))));
        assert!(textures.contains(&(String::from("whitePlane"), String::from("assets/whitePlane.png"))));
        assert!(textures.contains(&(String::from("bullet"), String::from("assets/bullet.png"))));

        assert_eq!(objects.len(), 3);
        assert_eq!(objects.get("plane"), Some(&TextureWrapper { width: 65, height: 65, padding: 1, frames: 3 }));
        assert_eq!(objects.get("whitePlane"), Some(&TextureWrapper { width: 65, height: 65, padding: 0, frames: 1 }));
    }
}
