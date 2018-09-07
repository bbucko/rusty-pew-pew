use cgmath::Vector2;
use game::enemy::EnemyState;
use game::player::PlayerState;

mod enemy;
pub mod factory;
pub mod player;
mod scene;


//pub type GameEvent = u8;
pub type Position = Vector2<f32>;
//pub type Translation = Vector2<f32>;
pub type Velocity = Vector2<f32>;
pub type Id = usize;

pub trait Renderer {
    fn render(&mut self, scene: &mut Vec<Option<GameObject>>);
    fn draw_texture(&mut self, texture_id: &str, position: Position);
    fn draw_frame(&mut self, texture_id: &str, position: Position, frame: u8);
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

#[derive(Debug)]
pub struct GameObject {
    pub id: Id,
    pub player: Option<PlayerState>,
    pub enemy: Option<EnemyState>,
}

impl GameObject {
    pub fn input(&mut self, input_state: &Vec<InputState>) {
        if let Some(ref mut player) = self.player {
            player.input(input_state)
        }

        if let Some(ref mut enemy) = self.enemy {
            enemy.input(input_state)
        }
    }

    pub fn draw(&mut self, renderer: &mut Renderer) {
        if let Some(ref mut player) = self.player {
            player.draw(renderer)
        }

        if let Some(ref mut enemy) = self.enemy {
            enemy.draw(renderer)
        }
    }

    pub fn update(&mut self) {
        if let Some(ref mut player) = self.player {
            player.update()
        }

        if let Some(ref mut enemy) = self.enemy {
            enemy.update()
        }
    }
}

pub struct Scene {
    pub position: Position,
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<u8>,
}

pub struct GameState {
    pub game_objects: Vec<Option<GameObject>>,
    pub scene: Scene,
}