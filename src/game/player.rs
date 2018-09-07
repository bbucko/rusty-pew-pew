use game::InputState;
use game::Position;
use game::Renderer;
use game::Velocity;

#[derive(Debug)]
pub struct PlayerState {
    position: Position,
    frame: u8,
    is_shooting: bool,
    velocity: Velocity,
}

impl PlayerState {
    pub fn new(position: Position) -> Self {
        PlayerState { position, frame: 0, is_shooting: false, velocity: Velocity::new(0.0, 0.0) }
    }

    pub fn input(&mut self, input_state: &Vec<InputState>) {
        let mut new_velocity = Velocity::new(0.0, 0.0);
        for input in input_state {
            match input {
                InputState::Up => new_velocity += Velocity::new(0.0, -2.0),
                InputState::Down => new_velocity += Velocity::new(0.0, 2.0),
                InputState::Left => new_velocity += Velocity::new(-2.0, 0.0),
                InputState::Right => new_velocity += Velocity::new(2.0, 0.0),
                InputState::Shoot => self.is_shooting = true,
                _ => self.velocity = Velocity::new(0.0, 0.0)
            }
        }
        self.velocity = new_velocity;
    }

    pub fn draw(&mut self, renderer: &mut Renderer) {
        renderer.draw_frame("plane", self.position, self.frame);
    }

    pub fn update(&mut self) {
        self.frame = (self.frame + 1) % 3;
        self.position += self.velocity;
    }
}