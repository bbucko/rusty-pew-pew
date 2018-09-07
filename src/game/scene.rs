use game::Entity;
use game::Position;
use game::Renderer;
use game::Scene;
use game::InputState;

impl Scene {
    pub fn new(game_objects: Vec<Box<Entity>>, tiles: Vec<u8>, width: u32, height: u32) -> Scene {
        Scene {
            position: Position::new(0.0, 0.0),
            game_objects,
            tiles,
            width,
            height,
        }
    }

    pub fn draw(&mut self, renderer: &mut Renderer) {
        for game_object in &mut self.game_objects {
            game_object.draw(renderer);
        }
    }

    pub fn update(&mut self) {
        for game_object in &mut self.game_objects {
            game_object.update();
        }
    }

    pub fn input(&mut self, input_state: &InputState) {
        for game_object in &mut self.game_objects {
            game_object.input(input_state);
        }
    }

}
