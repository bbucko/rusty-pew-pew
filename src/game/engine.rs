use game::Engine;
use game::GameObject;
use game::GameState;
use game::InputHandler;
use game::InputState;
use game::Renderer;
use game::states::BulletState;

impl<R, I> Engine<R, I> where R: Renderer, I: InputHandler {
    pub fn new(game_state: GameState, renderer: R, input_handler: I) -> Engine<R, I> {
        Engine { game_state, is_running: true, renderer, input_handler }
    }

    pub fn draw(&mut self) {
        self.renderer.render(&mut self.game_state.game_objects)
    }

    pub fn handle_input(&mut self) {
        let input_state = self.input_handler.capture();

        if Self::should_quit(&input_state) {
            self.is_running = false;
            return;
        }

        for game_object in &mut self.game_state.game_objects {
            if let Some(game_object) = game_object {
                game_object.input(&input_state);
            }
        }
    }

    pub fn update(&mut self) {
        Self::update_objects(&mut self.game_state.game_objects);

        Self::check_collisions(&mut self.game_state.game_objects);

        Self::remove_destroyed_objects(&mut self.game_state.game_objects);
    }

    fn check_collisions(game_objects: &[Option<GameObject>]) {
        for outer in game_objects {
            if let Some(game_object) = outer {
                for inner in game_objects {
                    if let Some(inner) = inner {
                        if inner.id != game_object.id {
                            game_object.check_collision(&inner);
                        }
                    }
                }
            }
        }
    }

    fn update_objects(game_objects: &mut Vec<Option<GameObject>>) {
        let mut bullets = Vec::new();

        for game_object in game_objects.iter_mut() {
            if let Some(obj) = game_object {
                if let Some(ref mut player) = obj.player {
                    player.update();

                    if player.is_shooting {
                        let new_bullet = GameObject { player: None, enemy: None, bullet: Some(BulletState::player_shoots(&player)), id: 111 };
                        bullets.push(Some(new_bullet));
                        player.is_shooting = false;
                    }
                }

                if let Some(ref mut enemy) = obj.enemy {
                    enemy.update();
                }
            }
        }

        game_objects.extend(bullets);
    }

    fn remove_destroyed_objects(game_objects: &mut [Option<GameObject>]) -> Vec<GameObject> {
        let mut removed = Vec::new();
        for game_object in game_objects {
            let remove = match game_object {
                Some(obj) => obj.is_destroyed(),
                _ => false,
            };

            if remove {
                removed.push(game_object.take().unwrap());
            }
        }

        removed
    }

    fn should_quit(inputs: &[InputState]) -> bool { inputs.contains(&InputState::Quit) }
}

mod test {
    use game::Engine;
    use game::GameObject;
    use game::InputHandler;
    use game::InputState;
    use game::Position;
    use game::Renderer;
    use game::states::PlayerState;

    struct MockRenderer {}

    struct MockInputHandler {}

    impl Renderer for MockRenderer {
        fn render(&mut self, _scene: &mut [Option<GameObject>]) {
            unimplemented!()
        }

        fn draw_texture(&mut self, _texture_id: &str, _position: Position) {
            unimplemented!()
        }

        fn draw_frame(&mut self, _texture_id: &str, _position: Position, _frame: u8) {
            unimplemented!()
        }
    }

    impl InputHandler for MockInputHandler {
        fn capture(&mut self) -> Vec<InputState> {
            unimplemented!()
        }
    }

    #[test]
    fn test_removal_of_empty_list() {
        //given
        let mut game_objects = vec![];

        //when
        Engine::<MockRenderer, MockInputHandler>::remove_destroyed_objects(&mut game_objects);

        //then
        assert_eq!(game_objects.len(), 0);
    }

    #[test]
    fn test_removal_of_list_of_none() {
        //given
        let mut game_objects = vec![None];

        //when
        Engine::<MockRenderer, MockInputHandler>::remove_destroyed_objects(&mut game_objects);

        //then
        assert_eq!(game_objects[0], None);
    }

    #[test]
    fn test_removal_of_list_of_some_non_removable_objects() {
        //given
        let obj = Some(GameObject { bullet: None, enemy: None, id: 1, player: None });
        let mut game_objects = vec![obj];

        //when
        Engine::<MockRenderer, MockInputHandler>::remove_destroyed_objects(&mut game_objects);

        //then
        assert_eq!(game_objects[0], Some(GameObject { bullet: None, enemy: None, id: 1, player: None }));
    }

    #[test]
    fn test_removal_of_list_of_some_removable_objects() {
        //given
        let mut player_state = PlayerState::new(Position::new(0.0, 0.0));
        player_state.is_destroyed = true;

        let obj = Some(GameObject { player: Some(player_state), enemy: None, bullet: None, id: 1 });
        let mut game_objects = vec![obj];

        //when
        Engine::<MockRenderer, MockInputHandler>::remove_destroyed_objects(&mut game_objects);

        //then
        assert_eq!(game_objects[0], None);
    }
}