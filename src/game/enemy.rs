use game::GameObject;
use game::Pos;
use game::Renderer;

#[derive(Debug)]
pub struct Enemy {
    position: Pos,
}

impl Enemy {
    pub fn new(position: Pos) -> Enemy {
        Enemy { position }
    }
}

impl GameObject for Enemy {
    fn draw(&self, renderer: &mut Renderer) {
        renderer.draw_texture("whitePlane", self.position);
    }

    fn update(&mut self) {}
}
