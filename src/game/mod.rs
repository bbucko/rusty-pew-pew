use cgmath::Vector2;
use std::time::SystemTime;

pub mod states;
mod game_object;
mod engine;
mod scene;
mod misc;

pub type Position = Vector2<f32>;
pub type Velocity = Vector2<f32>;
pub type Id = usize;

const RECT_PADDING: u32 = 2;

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

struct Rect {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

trait CollisionState {
    fn position(&self) -> Position;
    fn size(&self) -> (u32, u32);
    fn rect(&self) -> Rect {
        Rect::new(self.position().x, self.position().y, self.size().0, self.size().1)
    }
    fn rect_with_padding(&self) -> Rect {
        Rect::new(self.position().x, self.position().y, self.size().0 - RECT_PADDING, self.size().1 - RECT_PADDING)
    }

    fn is_colliding(&self, other: &CollisionState) -> bool {
        let a = self.rect_with_padding();
        let b = other.rect_with_padding();

        a.has_intersection(&b)
    }
}