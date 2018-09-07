use cgmath::Vector2;
use game::enemy::EnemyState;
use game::player::PlayerState;
use std::fmt::Debug;

mod enemy;
pub mod factory;
pub mod player;
mod scene;

pub type InputState = u8;
pub type Position = Vector2<f32>;
pub type Translation = Vector2<f32>;
//pub type Velocity = Vector2<f32>;
pub type Id = usize;

pub trait Renderer {
    fn render(&mut self, scene: &mut Vec<Option<GameObject>>);
    fn draw_texture(&mut self, texture_id: &str, position: Position);
    fn draw_frame(&mut self, texture_id: &str, position: Position, frame: u8);
}

pub trait InputHandler {
    fn capture(&mut self) -> Option<InputState>;
}

#[derive(Debug)]
pub struct GameObject {
    id: Id,
    player: Option<PlayerState>,
    enemy: Option<EnemyState>,
}

impl GameObject {
    pub fn id(&self) -> Id { self.id }
}

impl Entity for GameObject {
    fn input(&mut self, input_state: &InputState) {
        if let Some(ref mut player) = self.player {
            player.input(input_state)
        }

        if let Some(ref mut enemy) = self.enemy {
            enemy.input(input_state)
        }
    }

    fn draw(&mut self, renderer: &mut Renderer) {
        if let Some(ref mut player) = self.player {
            player.draw(renderer)
        }

        if let Some(ref mut enemy) = self.enemy {
            enemy.draw(renderer)
        }
    }

    fn update(&mut self, scene: &mut GameState) {
        if let Some(ref mut player) = self.player {
            player.update(scene)
        }

        if let Some(ref mut enemy) = self.enemy {
            enemy.update(scene)
        }
    }
}

pub trait Entity: Debug {
    fn input(&mut self, input_state: &InputState);

    fn draw(&mut self, renderer: &mut Renderer);

    fn update(&mut self, scene: &mut GameState);
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