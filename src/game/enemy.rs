use game::Entity;
use game::Id;
use game::InputState;
use game::Position;
use game::Renderer;

#[derive(Debug)]
pub struct Enemy {
    position: Position,
    id: Id,
}

impl Enemy {
    pub fn new(id: Id, position: Position) -> Enemy {
        Enemy { position, id }
    }
}

impl Entity for Enemy {
    fn id(&self) -> Id {
        self.id
    }

    fn input(&mut self, _input_state: &InputState) {}

    fn draw(&mut self, renderer: &mut Renderer) {
        renderer.draw_texture("whitePlane", self.position);
    }

    fn update(&mut self) {}
}
