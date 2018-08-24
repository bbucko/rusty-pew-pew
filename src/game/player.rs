use game::GameObject;
use game::Pos;
use game::Renderer;
use game::Transl;


#[derive(Debug)]
pub struct Player {
    position: Pos
}

impl Player {
    pub fn new() -> Self {
        Player {
            position: Pos::new(10.0, 0.0)
        }
    }

    pub fn up(&mut self) {
        self.position = self.position + Transl::new(0.0, 1.0);
    }
}

impl GameObject for Player {
    fn draw(&self, _: &Renderer) {}

    fn update(&mut self) {}
}

#[cfg(test)]
mod tests {
    use game::player::Player;
    use game::Pos;

    #[test]
    fn test_player_up() {
        let mut player = Player::new();
        assert_eq!(player.position, Pos::new(10.0, 0.0));

        player.up();
        assert_eq!(player.position, Pos::new(10.0, 1.0));
    }
}