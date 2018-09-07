use game::GameObject;
use game::GameState;
use game::Position;
use game::Scene;

impl GameState {
    pub fn new(game_objects: Vec<Option<GameObject>>, tiles: Vec<u8>, width: u32, height: u32) -> GameState {
        let scene = Scene { position: Position::new(0.0, 0.0), tiles, width, height };

        GameState {
            game_objects,
            scene,
        }
    }
}
