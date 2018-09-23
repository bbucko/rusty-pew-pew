use game::Level;
use game::Position;
use SCREEN_SIZE;

impl Level {
    pub fn new(width: u32, height: u32, tiles: Vec<u8>) -> Level {
        let position = Position::new(0, ((height * 32) - SCREEN_SIZE.1) as i32);
        Level {
            position,
            width,
            height,
            tiles,
        }
    }

    pub fn update(&mut self) {
        self.position += Position::new(0, -1)
    }
}
