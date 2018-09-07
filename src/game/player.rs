use game::Entity;
use game::Id;
use game::InputState;
use game::Position;
use game::Renderer;
use game::Translation;

#[derive(Debug)]
pub struct Player {
    position: Position,
    frame: u8,
    id: Id,
}

impl Player {
    pub fn new(id: Id, position: Position) -> Self {
        Player { position, frame: 0, id }
    }

    pub fn up(&mut self) {
        self.position += Translation::new(0.0, -1.0);
    }

    pub fn down(&mut self) {
        self.position += Translation::new(0.0, 1.0);
    }
}

impl Entity for Player {
    fn id(&self) -> Id {
        self.id
    }

    fn input(&mut self, input_state: &InputState) {
        match input_state {
            2 => self.up(),
            3 => self.down(),
            _ => ()
        }
    }

    fn draw(&mut self, renderer: &mut Renderer) {
        renderer.draw_frame("plane", self.position, self.frame);
    }

    fn update(&mut self) {
        self.frame = (self.frame + 1) % 3;
    }
}

#[cfg(test)]
mod tests {
    use game::player::Player;
    use game::Position;

    #[test]
    fn test_player_up() {
        let mut player = Player::new(0, Position::new(10.0, 0.0));
        assert_eq!(player.position, Position::new(10.0, 0.0));

        player.up();
        assert_eq!(player.position, Position::new(10.0, -1.0));
    }

    #[test]
    fn test_player_down() {
        let mut player = Player::new(0, Position::new(10.0, 0.0));
        assert_eq!(player.position, Position::new(10.0, 0.0));

        player.down();
        assert_eq!(player.position, Position::new(10.0, 1.0));
    }
}
