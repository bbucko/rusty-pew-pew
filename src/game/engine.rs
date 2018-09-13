use game::Engine;
use game::GameObject;
use game::InputHandler;
use game::InputState;
use game::Renderer;
use game::Scene;

impl<R, I> Engine<R, I> where R: Renderer, I: InputHandler {
    pub fn new(game_objects: Vec<Option<GameObject>>, scene: Scene, renderer: R, input_handler: I) -> Engine<R, I> {
        println!("Created engine");

        Engine { is_running: true, renderer, input_handler, scene, game_objects }
    }

    pub fn draw(&mut self) {
        self.renderer.render(&mut self.game_objects, &self.scene);
    }

    pub fn handle_input(&mut self) {
        let input_state = self.input_handler.capture();

        if Self::should_quit(&input_state) {
            self.is_running = false;
            return;
        }

        for game_object in &mut self.game_objects {
            if let Some(game_object) = game_object {
                game_object.input(&input_state);
            }
        }
    }

    pub fn update(&mut self) {
        Self::update_objects(&mut self.game_objects);

        Self::check_collisions(&mut self.game_objects);

        let _removed = Self::remove_destroyed_objects(&mut self.game_objects);

        self.scene.update();
    }

    fn check_collisions(game_objects: &mut Vec<Option<GameObject>>) {
        for i in 0..game_objects.len() {
            let (me, back) = {
                let (_, second) = game_objects.split_at_mut(i);
                let (me, rest) = second.split_first_mut().unwrap();
                (me, rest)
            };

            if let Some(me) = me {
                for candidate in back {
                    me.check_collision(candidate);
                }
            }
        }
    }

    fn update_objects(game_objects: &mut Vec<Option<GameObject>>) {
        let mut bullets = Vec::new();

        for game_object in game_objects.iter_mut() {
            if let Some(ref mut game_object) = game_object {
                if let Some(ref mut player) = game_object.player {
                    player.update();

                    if player.is_shooting {
                        let new_bullet = player.shoots();
                        bullets.push(Some(new_bullet));
                    }
                }

                if let Some(ref mut enemy) = game_object.enemy {
                    enemy.update();
                }

                if let Some(ref mut bullet) = game_object.bullet {
                    bullet.update();
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

#[cfg(test)]
mod tests {
    use game::Engine;
    use game::GameObject;
    use game::InputHandler;
    use game::InputState;
    use game::Position;
    use game::Renderer;
    use game::Scene;
    use game::states::PlayerState;

    struct MockRenderer {}

    struct MockInputHandler {}

    impl Renderer for MockRenderer {
        fn render(&mut self, _game_objects: &mut [Option<GameObject>], _scene: &Scene) {
            unimplemented!()
        }

        fn draw_texture(&mut self, _texture_id: &str, _position: Position, _scene: &Scene) {
            unimplemented!()
        }

        fn draw_frame(&mut self, _texture_id: &str, _position: Position, _frame: u8, _scene: &Scene) {
            unimplemented!()
        }
    }

    #[cfg(test)]
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

    #[test]
    fn test_collisions() {
        //given
        let mut v1 = vec![
            Some(GameObject { bullet: None, enemy: None, id: 1, player: None }),
            Some(GameObject { bullet: None, enemy: None, id: 2, player: None }),
            Some(GameObject { bullet: None, enemy: None, id: 3, player: None }),
            Some(GameObject { bullet: None, enemy: None, id: 4, player: None }),
            Some(GameObject { bullet: None, enemy: None, id: 5, player: None }),
            Some(GameObject { bullet: None, enemy: None, id: 6, player: None }),
            Some(GameObject { bullet: None, enemy: None, id: 7, player: None })
        ];

        //when
        Engine::<MockRenderer, MockInputHandler>::check_collisions(&mut v1);
    }

    #[test]
    fn test_collisions_with_empty_cells() {
        //given
        let mut v1 = vec![
            Some(GameObject { bullet: None, enemy: None, id: 1, player: None }),
            Some(GameObject { bullet: None, enemy: None, id: 2, player: None }),
            Some(GameObject { bullet: None, enemy: None, id: 3, player: None }),
            None,
            Some(GameObject { bullet: None, enemy: None, id: 4, player: None }),
            Some(GameObject { bullet: None, enemy: None, id: 5, player: None }),
            None,
            Some(GameObject { bullet: None, enemy: None, id: 6, player: None }),
            None,
            Some(GameObject { bullet: None, enemy: None, id: 7, player: None })
        ];

        //when
        Engine::<MockRenderer, MockInputHandler>::check_collisions(&mut v1);
    }
}