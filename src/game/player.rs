use game::GameObject;
use game::Id;
use game::Pos;
use game::Renderer;
use game::Transl;

#[derive(Debug)]
pub struct Player {
    position: Pos,
    frame: u8,
    id: Id,
}

impl Player {
    pub fn new(id: Id, position: Pos) -> Self {
        Player {
            position,
            frame: 0,
            id,
        }
    }

    pub fn up(&mut self) {
        self.position += Transl::new(0.0, -1.0);
    }

    pub fn down(&mut self) {
        self.position += Transl::new(0.0, 1.0);
    }
}

impl GameObject for Player {
    fn id(&self) -> Id {
        self.id
    }

    fn draw(&self, renderer: &mut Renderer) {
        renderer.draw_frame("plane", self.position, self.frame);
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
        let mut player = Player::new(0, Pos::new(10.0, 0.0));
        assert_eq!(player.position, Pos::new(10.0, 0.0));

        player.up();
        assert_eq!(player.position, Pos::new(10.0, -1.0));
    }

    #[test]
    fn test_player_down() {
        let mut player = Player::new(0, Pos::new(10.0, 0.0));
        assert_eq!(player.position, Pos::new(10.0, 0.0));

        player.down();
        assert_eq!(player.position, Pos::new(10.0, 1.0));
    }
}
