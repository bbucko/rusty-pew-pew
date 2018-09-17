use game::BulletState;
use game::CollisionState;
use game::EnemyState;
use game::GameObject;
use game::Id;
use game::InputState;
use game::ObjectType;
use game::PlayerState;
use game::Position;
use game::Renderer;
use game::Scene;
use game::Velocity;
use SCREEN_SIZE;
use std::collections::HashMap;
use std::num::ParseIntError;
use std::sync::atomic::{self, AtomicUsize};
use std::time::Duration;
use std::time::SystemTime;

lazy_static! {
    static ref OBJECT_COUNTER: AtomicUsize = <AtomicUsize>::new(1);
    static ref SHOOT_DELAY: Duration = Duration::new(0, 200_000_000);
}

pub fn create_game_object(properties: &HashMap<String, String>) -> Result<GameObject, String> {
    let object_type = match properties
        .get("type")
        .unwrap_or_else(|| panic!("Unknown type"))
        .as_str()
        {
            "Player" => ObjectType::Player,
            "Enemy" => ObjectType::Enemy,
            _ => ObjectType::Unknown,
        };

    let height = parse_int(properties, "height")?;
    let width = parse_int(properties, "width")?;
    let x = parse_int(properties, "x")? as i32;
    let y = parse_int(properties, "y")? as i32;

    Ok(GameObject::new(
        next_id(),
        Position::new(x, y),
        object_type,
        height,
        width,
    ))
}

fn next_id() -> Id {
    OBJECT_COUNTER.fetch_add(1, atomic::Ordering::SeqCst)
}

fn parse_int(properties: &HashMap<String, String>, attribute_name: &str) -> Result<u32, String> {
    properties
        .get(attribute_name)
        .unwrap_or_else(|| panic!("Missing: {:?}", attribute_name))
        .parse()
        .map_err(|e: ParseIntError| e.to_string())
}

impl PlayerState {
    pub fn new(id: Id, position: Position, width: u32, height: u32) -> Self {
        PlayerState {
            id,
            position,
            frame: 0,
            is_shooting: false,
            last_shot_date: SystemTime::now(),
            is_destroyed: false,
            velocity: Velocity::new(0, 0),
            width,
            height,
        }
    }

    pub fn input(&mut self, input_state: &[InputState]) {
        let mut new_velocity = Velocity::new(0, -1);
        for input in input_state {
            match input {
                InputState::Up => new_velocity += Velocity::new(0, -2),
                InputState::Down => new_velocity += Velocity::new(0, 2),
                InputState::Left => new_velocity += Velocity::new(-2, 0),
                InputState::Right => new_velocity += Velocity::new(2, 0),
                InputState::Shoot => self.is_shooting = true,
                _ => {}
            }
        }
        self.velocity = new_velocity;
    }

    pub fn draw(&mut self, renderer: &mut Renderer, scene: &Scene) {
        renderer.draw_frame("plane", self.position, self.frame, scene);
    }

    pub fn update(&mut self, scene: &Scene) -> Option<GameObject> {
        self.frame = (self.frame + 1) % 3;

        self.position += self.calculate_velocity(scene);

        if self.is_shooting && self.is_allowed_to_shoot() {
            Some(self.shoots())
        } else {
            None
        }
    }

    fn calculate_velocity(&self, scene: &Scene) -> Velocity {
        let (screen_width, screen_height) = SCREEN_SIZE;
        let mut fixed_velocity = self.velocity;
        let new_position = self.position + fixed_velocity;

        if new_position.x <= 0 || new_position.x + self.width as i32 >= screen_width as i32 {
            fixed_velocity.x = 0;
        }

        if new_position.y <= scene.position.y || new_position.y + self.height as i32 >= scene.position.y + screen_height as i32 {
            fixed_velocity.y = -1;
        }

        fixed_velocity
    }

    fn is_allowed_to_shoot(&self) -> bool {
        let now = SystemTime::now();
        let duration = now.duration_since(self.last_shot_date).unwrap();
        duration.ge(&SHOOT_DELAY)
    }

    fn shoots(&mut self) -> GameObject {
        self.is_shooting = false;
        self.last_shot_date = SystemTime::now();

        GameObject::new_bullet(next_id(), self.position, ObjectType::Player, self.id)
    }
}

impl EnemyState {
    pub fn new(id: Id, position: Position, width: u32, height: u32) -> EnemyState {
        EnemyState {
            id,
            position,
            is_destroyed: false,
            width,
            height,
            velocity: Velocity::new(1, -1),
        }
    }

    pub fn input(&mut self, _input_state: &[InputState]) {}

    pub fn draw(&mut self, renderer: &mut Renderer, scene: &Scene) {
        renderer.draw_texture("whitePlane", self.position, scene);
    }

    pub fn update(&mut self) -> Option<GameObject> {
        if self.position.x == 0 as i32 {
            self.velocity.x = 1;
        } else if self.position.x + self.width as i32 == SCREEN_SIZE.0 as i32 {
            self.velocity.x = -1;
        }
        self.position += self.velocity;
        None
    }
}

impl BulletState {
    pub fn _enemy_bullet(enemy: &EnemyState) -> BulletState {
        BulletState {
            shooter_type: ObjectType::Enemy,
            shooter_id: enemy.id,
            position: enemy.position,
            velocity: Velocity::new(0, 8),
            is_destroyed: false,
        }
    }

    pub fn draw(&mut self, renderer: &mut Renderer, scene: &Scene) {
        renderer.draw_texture("bullet", self.position, scene);
    }

    pub fn update(&mut self) -> Option<GameObject> {
        self.position += self.velocity;
        None
    }

    pub fn is_fired_by(&self, shooter: &GameObject) -> bool {
        self.shooter_id == shooter.id
    }
}

impl CollisionState for BulletState {
    fn position(&self) -> Position {
        self.position
    }

    fn size(&self) -> (u32, u32) {
        (32, 32)
    }
}

impl CollisionState for PlayerState {
    fn position(&self) -> Position {
        self.position
    }

    fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

impl CollisionState for EnemyState {
    fn position(&self) -> Position {
        self.position
    }

    fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
