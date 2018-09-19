use cgmath::Vector2;
use std::time::SystemTime;

mod engine;
mod game_object;
mod misc;
mod level;
pub mod states;

pub type Position = Vector2<i32>;
pub type Velocity = Vector2<i32>;
pub type Id = usize;

pub struct Engine<R: Renderer, I: InputHandler> {
    pub is_running: bool,
    renderer: R,
    input_handler: I,
    level: Level,
    game_objects: Vec<Option<GameObject>>,
}

pub trait Renderer {
    fn render(&mut self, game_objects: &mut [Option<GameObject>], level: &Level);
    fn draw_texture(&mut self, texture_id: &str, position: Position, level: &Level);
    fn draw_frame(&mut self, texture_id: &str, position: Position, frame: u8, level: &Level);
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
pub struct Level {
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
    velocity: Velocity,
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

#[derive(Debug)]
struct Rect {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

trait CollisionState {
    fn position(&self) -> Position;
    fn size(&self) -> (u32, u32);
    fn collision_padding(&self) -> (u32, u32);

    fn collision_rect(&self) -> Rect {
        let (padding_horizontal, padding_vertical) = self.collision_padding();
        let position = self.position();
        let (width, height) = self.size();

        Rect::new(
            position.x + padding_horizontal as i32,
            position.y + padding_vertical as i32,
            width - padding_horizontal,
            height - padding_vertical,
        )
    }

    fn is_colliding(&self, other: &CollisionState) -> bool {
        let a = self.collision_rect();
        let b = other.collision_rect();

        a.has_intersection(&b)
    }
}
