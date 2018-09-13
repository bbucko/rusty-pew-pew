use game::Position;
use game::Scene;

impl Scene {
    pub fn new(width: u32, height: u32, tiles: Vec<u8>) -> Scene {
        let position = Position::new(0.0, ((height * 32) - 480) as f32);
        println!("Creating scene: {:?}", position);
        Scene { position, width, height, tiles }
    }

    pub fn update(&mut self) {
        self.position += Position::new(0.0, 0.0)
    }
}