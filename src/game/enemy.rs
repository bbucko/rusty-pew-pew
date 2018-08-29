use game::GameObject;
use game::Pos;
use game::Renderer;

#[derive(Debug)]
pub struct Enemy {
    position: Pos
}

impl Enemy {
    pub fn new() -> Enemy {
        Enemy {
            position: Pos::new(0.0, 0.0)
        }
    }
}

impl GameObject for Enemy {
    fn draw(&self, renderer: &mut Renderer) {
        renderer.draw_texture("assets/bullet.png", self.position);
    }

    fn update(&mut self) {}
}
