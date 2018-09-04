use game::GameObject;
use game::Id;
use game::Pos;
use game::Renderer;

#[derive(Debug)]
pub struct Enemy {
    position: Pos,
    id: Id,
}

impl Enemy {
    pub fn new(id: Id, position: Pos) -> Enemy {
        Enemy { position, id }
    }
}

impl GameObject for Enemy {
    fn id(&self) -> Id {
        self.id
    }

    fn draw(&self, renderer: &mut Renderer) {
        renderer.draw_texture("whitePlane", self.position);
    }

    fn update(&mut self) {}
}
