use cgmath::Vector2;
use game::states::BulletState;
use game::states::EnemyState;
use game::states::PlayerState;

pub mod states;
mod game_object;
mod engine;
mod scene;

pub type Position = Vector2<f32>;
pub type Velocity = Vector2<f32>;
pub type Id = usize;

pub trait Renderer {
    fn render(&mut self, game_objects: &mut [Option<GameObject>], scene: &Scene);
    fn draw_texture(&mut self, texture_id: &str, position: Position, scene: &Scene);
    fn draw_frame(&mut self, texture_id: &str, position: Position, frame: u8, scene: &Scene);
}

#[derive(Debug, PartialEq)]
pub enum InputState {
    Up,
    Down,
    Left,
    Right,
    Shoot,
    Quit,
}

pub trait InputHandler {
    fn capture(&mut self) -> Vec<InputState>;
}

#[derive(Debug, PartialEq)]
pub struct GameObject {
    pub id: Id,
    pub player: Option<PlayerState>,
    pub enemy: Option<EnemyState>,
    pub bullet: Option<BulletState>
}


pub struct Engine<R: Renderer, I: InputHandler> {
    pub is_running: bool,
    renderer: R,
    input_handler: I,
    scene: Scene,
    game_objects: Vec<Option<GameObject>>,
}

#[allow(dead_code)]
pub struct Scene {
    pub position: Position,
    width: u32,
    height: u32,
    tiles: Vec<u8>,
}