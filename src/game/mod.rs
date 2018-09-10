use cgmath::Vector2;
use game::states::BulletState;
use game::states::EnemyState;
use game::states::PlayerState;

pub mod states;
mod engine;
mod scene;

pub type Position = Vector2<f32>;
pub type Velocity = Vector2<f32>;
pub type Id = usize;

pub trait Renderer {
    fn render(&mut self, game_objects: &mut [Option<GameObject>]);
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

#[derive(Debug, PartialEq)]
pub struct GameObject {
    pub id: Id,
    pub player: Option<PlayerState>,
    pub enemy: Option<EnemyState>,
    pub bullet: Option<BulletState>,
}

impl GameObject {
    pub fn input(&mut self, input_state: &[InputState]) {
        if let Some(ref mut player) = self.player {
            player.input(input_state);
        }

        if let Some(ref mut enemy) = self.enemy {
            enemy.input(input_state);
        }
    }

    pub fn draw(&mut self, renderer: &mut Renderer) {
        if let Some(ref mut player) = self.player {
            player.draw(renderer);
        }

        if let Some(ref mut enemy) = self.enemy {
            enemy.draw(renderer);
        }
    }

    fn is_destroyed(&self) -> bool {
        if let Some(ref player) = self.player {
            return player.is_destroyed;
        }

        if let Some(ref enemy) = self.enemy {
            return enemy.is_destroyed;
        }

        false
    }

    fn check_collision(&self, object: &GameObject) -> bool {
        println!("Checking collision: {:?} with {:?}", self, object);
        false
    }
}

pub struct Engine<R: Renderer, I: InputHandler> {
    game_state: GameState,
    renderer: R,
    input_handler: I,
    pub is_running: bool,
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