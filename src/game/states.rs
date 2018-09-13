use game::GameObject;
use game::Id;
use game::InputState;
use game::Position;
use game::Renderer;
use game::Scene;
use game::Velocity;
use std::collections::HashMap;
use std::num::ParseFloatError;
use std::sync::atomic::{self, AtomicUsize};
use std::time::Duration;
use std::time::SystemTime;

lazy_static! {
    static ref OBJECT_COUNTER: AtomicUsize = <AtomicUsize>::new(1);
    static ref SHOOT_DELAY: Duration = Duration::new(0, 200_000_000);
}

pub fn create_game_object(properties: &HashMap<String, String>) -> Result<GameObject, String> {
    let object_type = properties.get("type").unwrap_or_else(|| panic!("Unknown type")).as_str();

    let x = parse_float(properties, "x")?;
    let y = parse_float(properties, "y")?;

    let position = Position::new(x, y);

    let id = next_id();
    let mut default_object = GameObject { id, player: None, enemy: None, bullet: None };

    match object_type {
        "Enemy" => default_object.enemy = Some(EnemyState::new(position)),
        "Player" => default_object.player = Some(PlayerState::new(position)),
        _ => panic!("unknown type: {:?}", object_type),
    }

    Ok(default_object)
}

fn next_id() -> Id { OBJECT_COUNTER.fetch_add(1, atomic::Ordering::SeqCst) }

fn parse_float(properties: &HashMap<String, String>, attribute_name: &str) -> Result<f32, String> {
    properties
        .get(attribute_name)
        .unwrap_or_else(|| panic!("Missing: {:?}", attribute_name))
        .parse()
        .map_err(|e: ParseFloatError| e.to_string())
}

#[derive(Debug, PartialEq)]
pub struct PlayerState {
    position: Position,
    frame: u8,
    pub is_shooting: bool,
    pub is_destroyed: bool,
    last_shot_date: SystemTime,
    velocity: Velocity,
}

impl PlayerState {
    pub fn new(position: Position) -> Self {
        PlayerState { position, frame: 0, is_shooting: false, last_shot_date: SystemTime::now(), is_destroyed: false, velocity: Velocity::new(0.0, 0.0) }
    }

    pub fn input(&mut self, input_state: &[InputState]) {
        let mut new_velocity = Velocity::new(0.0, 0.0);
        for input in input_state {
            match input {
                InputState::Up => new_velocity += Velocity::new(0.0, -2.0),
                InputState::Down => new_velocity += Velocity::new(0.0, 2.0),
                InputState::Left => new_velocity += Velocity::new(-2.0, 0.0),
                InputState::Right => new_velocity += Velocity::new(2.0, 0.0),
                InputState::Shoot => if self.is_allowed_to_shoot() { self.is_shooting = true },
                _ => self.velocity = Velocity::new(0.0, 0.0)
            }
        }
        self.velocity = new_velocity;
    }

    pub fn draw(&mut self, renderer: &mut Renderer, scene: &Scene) {
        renderer.draw_frame("plane", self.position, self.frame, scene);
    }

    pub fn update(&mut self) {
        self.frame = (self.frame + 1) % 3;
        self.position += self.velocity;
    }

    pub fn is_allowed_to_shoot(&self) -> bool {
        let now = SystemTime::now();
        let duration = now.duration_since(self.last_shot_date).unwrap();
        duration.ge(&SHOOT_DELAY)
    }

    pub fn shoots(&mut self) -> GameObject {
        self.is_shooting = false;
        self.last_shot_date = SystemTime::now();

        GameObject {
            id: next_id(),
            player: None,
            enemy: None,
            bullet: Some(BulletState::player_bullet(self)),
        }
    }
}


#[derive(Debug, PartialEq)]
pub struct EnemyState {
    position: Position,
    pub is_destroyed: bool,
}

impl EnemyState {
    pub fn new(position: Position) -> EnemyState {
        EnemyState { position, is_destroyed: false }
    }

    pub fn input(&mut self, _input_state: &[InputState]) {}

    pub fn draw(&mut self, renderer: &mut Renderer, scene: &Scene) {
        renderer.draw_texture("whitePlane", self.position, scene);
    }

    pub fn update(&mut self) {}
}

#[derive(Debug, PartialEq)]
pub struct BulletState {
    position: Position,

}

impl BulletState {
    pub fn player_bullet(player: &PlayerState) -> BulletState {
        BulletState { position: player.position + Position::new(0.0, -35.0) }
    }

    pub fn _enemy_bullet(enemy: &EnemyState) -> BulletState {
        BulletState { position: enemy.position }
    }

    pub fn draw(&mut self, renderer: &mut Renderer, scene: &Scene) {
        renderer.draw_texture("bullet", self.position, scene);
    }

    pub fn update(&mut self) {
        self.position += Position::new(0.0, -4.0);
    }
}