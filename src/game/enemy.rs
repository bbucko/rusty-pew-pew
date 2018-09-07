use game::Entity;
use game::InputState;
use game::Position;
use game::Renderer;
use game::GameState;

#[derive(Debug)]
pub struct EnemyState {
    position: Position,
}

impl EnemyState {
    pub fn new(position: Position) -> EnemyState {
        EnemyState { position }
    }
}

impl Entity for EnemyState {
    fn input(&mut self, _input_state: &InputState) {}

    fn draw(&mut self, renderer: &mut Renderer) {
        renderer.draw_texture("whitePlane", self.position);
    }

    fn update(&mut self, _scene: &mut GameState) {}
}
