use game::Position;
use game::Scene;
use SCREEN_SIZE;

impl Scene {
    pub fn new(width: u32, height: u32, tiles: Vec<u8>) -> Scene {
        let position = Position::new(0, ((height * 32) - SCREEN_SIZE.1) as i32);
        Scene {
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
