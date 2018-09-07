use game::GameState;
use game::InputState;
use game::Position;
use game::Renderer;

#[derive(Debug)]
pub struct EnemyState {
    position: Position,
}

impl EnemyState {
    pub fn new(position: Position) -> EnemyState {
        EnemyState { position }
    }

    pub fn input(&mut self, _input_state: &Vec<InputState>) {}

    pub fn draw(&mut self, renderer: &mut Renderer) {
        renderer.draw_texture("whitePlane", self.position);
    }

    pub fn _update(&mut self, _scene: &mut GameState) {}

    pub fn update(&mut self) {}
}
