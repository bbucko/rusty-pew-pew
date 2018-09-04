use cgmath::Vector2;
use game::player::Player;
use std::fmt::Debug;

mod enemy;
pub mod factory;
pub mod player;

pub type Event = u8;
pub type Pos = Vector2<f32>;
pub type Transl = Vector2<f32>;
pub type Id = usize;

pub trait Renderer {
    fn render(&mut self, scene: &mut Scene);
    fn draw_texture(&mut self, texture_id: &str, position: Pos);
    fn draw_frame(&mut self, texture_id: &str, position: Pos, frame: u8);
}

pub trait InputHandler {
    fn handle(&mut self) -> Vec<Event>;
}

pub trait GameObject: Debug {
    fn id(&self) -> Id;

    fn draw(&self, renderer: &mut Renderer);

    fn update(&mut self);
}

pub struct Scene {
    player: Player,
    game_objects: Vec<Box<GameObject>>,
}

pub struct Engine<R: Renderer, IH: InputHandler> {
    running: bool,
    video: R,
    input_handler: IH,
    scene: Scene,
}

impl Scene {
    pub fn new(player: Player, game_objects: Vec<Box<GameObject>>) -> Scene {
        Scene { player, game_objects }
    }

    pub fn draw(&mut self, renderer: &mut Renderer) {
        self.player.draw(renderer);

        for game_object in &mut self.game_objects {
            game_object.draw(renderer);
        }
    }

    pub fn update(&mut self) {
        self.player.update();

        for game_object in &mut self.game_objects {
            game_object.update();
        }
    }
}

impl<T, U> Engine<T, U>
where
    T: Renderer,
    U: InputHandler,
{
    pub fn new(video: T, input_handler: U, scene: Scene) -> Self {
        Engine {
            running: true,
            video,
            input_handler,
            scene,
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn handle_events(&mut self) {
        let events = self.input_handler.handle();

        for event in events {
            if event == 1 {
                self.running = false;
            } else {
                self.scene.player.down();
            }
        }
    }

    pub fn update(&mut self) {
        self.scene.update();
    }

    pub fn render(&mut self) {
        self.video.render(&mut self.scene);
    }
}
