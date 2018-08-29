use game::GameObject;
use game::Pos;
use game::Renderer;
use game::Transl;

#[derive(Debug)]
pub struct Player {
    position: Pos,
    frame: u8,
}

impl Player {
    pub fn new(position: Pos) -> Self {
        let frame = 0;
        Player { position, frame }
    }

    pub fn up(&mut self) {
        self.position = self.position + Transl::new(0.0, 1.0);
    }
}

impl GameObject for Player {
    fn draw(&self, renderer: &mut Renderer) {
        renderer.draw_frame("assets/plane.png", self.position, self.frame);
    }

    fn update(&mut self) {
        self.frame = (self.frame + 1) % 3;
    }
}

#[cfg(test)]
mod tests {
    use game::player::Player;
    use game::Pos;

    #[test]
    fn test_player_up() {
        let mut player = Player::new(Pos::new(10.0, 0.0));
        assert_eq!(player.position, Pos::new(10.0, 0.0));

        player.up();
        assert_eq!(player.position, Pos::new(10.0, 1.0));
    }
}
