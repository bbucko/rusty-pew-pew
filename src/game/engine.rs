use game::Engine;
use game::GameObject;
use game::InputHandler;
use game::InputState;
use game::Renderer;
use game::Scene;
use std::mem;

impl<R, I> Engine<R, I> where R: Renderer, I: InputHandler {
    pub fn new(game_objects: Vec<Option<GameObject>>, scene: Scene, renderer: R, input_handler: I) -> Engine<R, I> {
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
                game_object.handle_input(&input_state);
            }
        }
    }

    pub fn update(&mut self) {
        self.update_objects();
        self.check_collisions();
        self.remove_destroyed_objects();

        self.scene.update();
    }

    fn update_objects(&mut self) {
        let mut new_object = Vec::new();

        for game_object in self.game_objects.iter_mut() {
            if let Some(ref mut game_object) = game_object {
                game_object.update(&mut new_object, &self.scene);
            }
        }

        self.add_new_objects(new_object);
    }

    fn add_new_objects(&mut self, new_objects: Vec<Option<GameObject>>) {
        let mut next_new_object = new_objects.into_iter()
            .filter(|obj| obj.is_some());

        for i in 0..self.game_objects.len() {
            if self.game_objects[i] == None {
                if let Some(option) = next_new_object.next() {
                    mem::replace(&mut self.game_objects[i], option);
                    continue;
                }
                break;
            }
        }

        let new_objects = next_new_object
            .filter(|s| s.is_some())
            .collect::<Vec<Option<GameObject>>>();

        self.game_objects.extend(new_objects);
    }

    fn check_collisions(&mut self) {
        for i in 0..self.game_objects.len() {
            let (me, tail) = self.game_objects[i..].split_first_mut().unwrap();
            if let Some(me) = me {
                for candidate in tail {
                    if let Some(candidate) = candidate {
                        me.check_collision(candidate);
                    }
                }
            }
        }
    }

    fn remove_destroyed_objects(&mut self) {
        for game_object in self.game_objects.iter_mut() {
            let should_remove = match game_object {
                Some(obj) => obj.is_destroyed(),
                _ => false,
            };

            if should_remove {
                game_object.take();
            }
        }
    }

    fn should_quit(inputs: &[InputState]) -> bool {
        inputs.contains(&InputState::Quit)
    }
}

#[cfg(test)]
mod tests {
    use game::Engine;
    use game::GameObject;
    use game::Id;
    use game::InputHandler;
    use game::InputState;
    use game::ObjectType;
    use game::PlayerState;
    use game::Position;
    use game::Renderer;
    use game::Scene;

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
        let game_objects = vec![];
        let scene = create_fake_scene();
        let mut engine = Engine::new(game_objects, scene, MockRenderer {}, MockInputHandler {});

        //when
        engine.remove_destroyed_objects();

        //then
        assert_eq!(engine.game_objects.len(), 0);
    }

    #[test]
    fn test_removal_of_list_of_none() {
        //given
        let game_objects = vec![None];
        let scene = create_fake_scene();
        let mut engine = Engine::new(game_objects, scene, MockRenderer {}, MockInputHandler {});

        //when
        engine.remove_destroyed_objects();

        //then
        assert_eq!(engine.game_objects[0], None);
    }

    #[test]
    fn test_removal_of_list_of_some_non_removable_objects() {
        //given
        let obj = Some(create_fake_object(1));
        let game_objects = vec![obj];
        let scene = create_fake_scene();
        let mut engine = Engine::new(game_objects, scene, MockRenderer {}, MockInputHandler {});

        //when
        engine.remove_destroyed_objects();

        //then
        assert_eq!(engine.game_objects[0], Some(create_fake_object(1)));
    }

    #[test]
    fn test_removal_of_list_of_some_removable_objects() {
        //given
        let mut player_state = PlayerState::new(1, Position::new(0.0, 0.0), 0, 0);
        player_state.is_destroyed = true;

        let obj = Some(GameObject { player: Some(player_state), enemy: None, bullet: None, id: 1, object_type: ObjectType::Player });
        let game_objects = vec![obj];
        let scene = create_fake_scene();
        let mut engine = Engine::new(game_objects, scene, MockRenderer {}, MockInputHandler {});

        //when
        engine.remove_destroyed_objects();

        //then
        assert_eq!(engine.game_objects[0], None);
    }

    #[test]
    fn test_collisions() {
        //given
        let game_objects = vec![
            Some(create_fake_object(1)),
            Some(create_fake_object(2)),
            Some(create_fake_object(3)),
            Some(create_fake_object(4)),
            Some(create_fake_object(5)),
            Some(create_fake_object(6)),
            Some(create_fake_object(7)),
        ];
        let scene = create_fake_scene();
        let mut engine = Engine::new(game_objects, scene, MockRenderer {}, MockInputHandler {});

        //when
        engine.check_collisions();

        //then
        //fixme missing assers
    }

    #[test]
    fn test_collisions_with_empty_cells() {
        //given
        let game_objects = vec![
            Some(create_fake_object(1)),
            Some(create_fake_object(2)),
            Some(create_fake_object(3)),
            None,
            Some(create_fake_object(4)),
            Some(create_fake_object(5)),
            None,
            Some(create_fake_object(6)),
            None,
            Some(create_fake_object(7)),
        ];
        let scene = create_fake_scene();
        let mut engine = Engine::new(game_objects, scene, MockRenderer {}, MockInputHandler {});

        //when
        engine.check_collisions();

        //then
        //fixme missing assers
    }

    #[test]
    fn test_updating_objects() {
        //given
        let game_objects = vec![
            None
        ];
        let new_objects = vec![
            Some(create_fake_object(1)),
        ];
        let scene = create_fake_scene();
        let mut engine = Engine::new(game_objects, scene, MockRenderer {}, MockInputHandler {});

        //when
        engine.add_new_objects(new_objects);

        assert_eq!(engine.game_objects, vec![Some(create_fake_object(1))]);
    }

    #[test]
    fn test_updating_objects_with_expand() {
        //given
        let game_objects = vec![
            Some(create_fake_object(0))
        ];
        let new_objects = vec![
            Some(create_fake_object(1)),
            Some(create_fake_object(2)),
        ];
        let scene = create_fake_scene();
        let mut engine = Engine::new(game_objects, scene, MockRenderer {}, MockInputHandler {});

        //when
        engine.add_new_objects(new_objects);

        assert_eq!(engine.game_objects, vec![Some(create_fake_object(0)), Some(create_fake_object(1)), Some(create_fake_object(2)), ]);
    }

    #[test]
    fn test_updating_objects_with_none_and_expand() {
        //given
        let game_objects = vec![
            None,
            Some(create_fake_object(1))];
        let new_objects = vec![
            Some(create_fake_object(2)),
            Some(create_fake_object(3)),
        ];
        let scene = create_fake_scene();
        let mut engine = Engine::new(game_objects, scene, MockRenderer {}, MockInputHandler {});

        //when
        engine.add_new_objects(new_objects);

        assert_eq!(engine.game_objects, vec![Some(create_fake_object(2)), Some(create_fake_object(1)), Some(create_fake_object(3))]);
    }

    #[test]
    fn test_updating_objects_with_none_in_new_objects_and_expand() {
        //given
        let game_objects = vec![
            None,
            Some(create_fake_object(1))];

        let new_objects = vec![
            Some(create_fake_object(2)),
            None
        ];
        let scene = create_fake_scene();
        let mut engine = Engine::new(game_objects, scene, MockRenderer {}, MockInputHandler {});

        //when
        engine.add_new_objects(new_objects);

        assert_eq!(engine.game_objects, vec![Some(create_fake_object(2)), Some(create_fake_object(1))]);
    }

    fn create_fake_object(id: Id) -> GameObject { GameObject { id, object_type: ObjectType::Unknown, player: None, enemy: None, bullet: None } }

    fn create_fake_scene() -> Scene { Scene::new(1000, 1000, vec![]) }
}