use cgmath::Vector2;
use std::fmt::Debug;

mod enemy;
mod engine;
pub mod factory;
pub mod player;
mod scene;

pub type InputState = u8;
pub type Position = Vector2<f32>;
pub type Translation = Vector2<f32>;
pub type Velocity = Vector2<f32>;
pub type Id = usize;

pub trait Renderer {
    fn render(&mut self, scene: &mut Scene);
    fn draw_texture(&mut self, texture_id: &str, position: Position);
    fn draw_frame(&mut self, texture_id: &str, position: Position, frame: u8);
}

pub trait InputHandler {
    fn events(&mut self) -> Option<InputState>;
}

pub trait Entity: Debug {
    fn id(&self) -> Id;

    fn input(&mut self, input_state: &InputState);

    fn draw(&mut self, renderer: &mut Renderer);

    fn update(&mut self);
}

pub struct Scene {
    pub position: Position,
    pub game_objects: Vec<Box<Entity>>,
    pub tiles: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

pub struct Engine<R: Renderer, IH: InputHandler> {
    running: bool,
    video: R,
    input_handler: IH,
    scene: Scene,
}