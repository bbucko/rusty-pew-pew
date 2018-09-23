use game::Level;
use game::Position;
use game::Renderer;
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
        if self.position.y > 0 {
            self.position += Position::new(0, -1)
        }
    }

    pub fn draw(&self, renderer: &mut Renderer) {
        let tiles = &self.tiles;
        for rows in 0..self.height {
            for cols in 0..self.width {
                let id = rows * self.width + cols;
                let tile_id = tiles[id as usize];
                if tile_id != 0 {
                    let x = cols as i32 * 32;
                    let y = rows as i32 * 32;
                    let tile_position = Position::new(x, y) - self.position;
                    renderer.draw_tile("tiles", tile_position, tile_id - 1);
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use game::Level;
    use game::Position;
    use game::Renderer;

    struct MockRenderer {
        pub interactions: Vec<String>
    }

    impl MockRenderer {
        fn new() -> MockRenderer {
            MockRenderer { interactions: vec![] }
        }
    }

    impl Renderer for MockRenderer {
        fn clear_scene(&mut self) {
            self.interactions.push(String::from("clear_scene"));
        }

        fn draw_scene(&mut self) {
            self.interactions.push(String::from("draw_scene"));
        }

        fn draw_texture(&mut self, _texture_id: &str, _position: Position, _level: &Level) {
            self.interactions.push(String::from("draw_texture"));
        }

        fn draw_tile(&mut self, _texture_id: &str, _position: Position, _tile_id: u8) {
            self.interactions.push(String::from("draw_tile"));
        }

        fn draw_frame(&mut self, _texture_id: &str, _position: Position, _level: &Level) {
            self.interactions.push(String::from("draw_frame"));
        }
    }

    #[test]
    fn test_one_tick_of_update_if_not_allowed() {
        //given
        let mut scene = Level {
            position: Position::new(0, 0),
            width: 0,
            height: 0,
            tiles: Vec::new(),
        };

        //when
        scene.update();

        //then
        assert_eq!(scene.position, Position::new(0, 0))
    }

    #[test]
    fn test_one_tick_of_update() {
        //given
        let mut scene = Level {
            position: Position::new(0, 10),
            width: 0,
            height: 0,
            tiles: Vec::new(),
        };

        //when
        scene.update();

        //then
        assert_eq!(scene.position, Position::new(0, 9))
    }

    #[test]
    fn test_draw() {
        //given
        let mut renderer = MockRenderer::new();
        let mut tiles = vec![0; 100];
        tiles[0] = 1;
        tiles[10] = 1;
        tiles[30] = 2;
        tiles[40] = 3;
        tiles[50] = 4;
        tiles[60] = 5;

        let scene = Level {
            position: Position::new(0, 0),
            width: 10,
            height: 10,
            tiles,
        };

        //when
        scene.draw(&mut renderer);

        //then
        assert_eq!(renderer.interactions.len(), 6);
    }
}

