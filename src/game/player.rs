use game::Entity;
use game::InputState;
use game::Position;
use game::Renderer;
use game::GameState;
use game::Translation;

#[derive(Debug)]
pub struct PlayerState {
    position: Position,
    frame: u8,
}

impl PlayerState {
    pub fn new(position: Position) -> Self {
        PlayerState { position, frame: 0 }
    }

    pub fn up(&mut self) {
        self.position += Translation::new(0.0, -1.0);
    }

    pub fn down(&mut self) {
        self.position += Translation::new(0.0, 1.0);
    }
}

impl Entity for PlayerState {
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

    fn update(&mut self, _scene: &mut GameState) {
        self.frame = (self.frame + 1) % 3;
    }
}

#[cfg(test)]
mod tests {
    use game::player::PlayerState;
    use game::Position;

    #[test]
    fn test_player_up() {
        let mut player = PlayerState::new(Position::new(10.0, 0.0));
        assert_eq!(player.position, Position::new(10.0, 0.0));

        player.up();
        assert_eq!(player.position, Position::new(10.0, -1.0));
    }

    #[test]
    fn test_player_down() {
        let mut player = PlayerState::new(Position::new(10.0, 0.0));
        assert_eq!(player.position, Position::new(10.0, 0.0));

        player.down();
        assert_eq!(player.position, Position::new(10.0, 1.0));
    }
}
