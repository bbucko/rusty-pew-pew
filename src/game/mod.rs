use cgmath::Vector2;
use sdl2::rect::Rect;
use std::time::SystemTime;

pub mod states;
mod game_object;
mod engine;
mod scene;

pub type Position = Vector2<f32>;
pub type Velocity = Vector2<f32>;
pub type Id = usize;

pub struct Engine<R: Renderer, I: InputHandler> {
    pub is_running: bool,
    renderer: R,
    input_handler: I,
    scene: Scene,
    game_objects: Vec<Option<GameObject>>,
}

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


#[allow(dead_code)]
pub struct Scene {
    pub position: Position,
    width: u32,
    height: u32,
    tiles: Vec<u8>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ObjectType {
    Player,
    Enemy,
    Bullet,
    Unknown,
}

#[derive(PartialEq)]
pub struct GameObject {
    pub id: Id,
    pub object_type: ObjectType,
    pub player: Option<PlayerState>,
    pub enemy: Option<EnemyState>,
    pub bullet: Option<BulletState>,
}

#[derive(Debug, PartialEq)]
pub struct PlayerState {
    id: Id,
    position: Position,
    frame: u8,
    pub is_shooting: bool,
    pub is_destroyed: bool,
    last_shot_date: SystemTime,
    velocity: Velocity,
    width: u32,
    height: u32,
}

#[derive(Debug, PartialEq)]
pub struct EnemyState {
    id: Id,
    position: Position,
    pub is_destroyed: bool,
    width: u32,
    height: u32,
}

#[derive(Debug, PartialEq)]
pub struct BulletState {
    position: Position,
    velocity: Velocity,
    shooter_type: ObjectType,
    shooter_id: Id,
    pub is_destroyed: bool,
}

trait CollisionState {
    fn position(&self) -> Position;
    fn size(&self) -> (u32, u32);

    fn is_colliding(&self, with: &CollisionState) -> bool {
        let padding = 10;
        let a = Rect::new(self.position().x as i32, self.position().y as i32, self.size().0 - padding, self.size().1 - padding);
        let b = Rect::new(with.position().x as i32, with.position().y as i32, with.size().0 - padding, with.size().1 - padding);
        a.has_intersection(b)
    }
}