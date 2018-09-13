use game::GameObject;
use game::InputState;
use game::Renderer;
use game::Scene;

impl GameObject {
    pub fn input(&mut self, input_state: &[InputState]) {
        if let Some(ref mut player) = self.player {
            player.input(input_state);
        }

        if let Some(ref mut enemy) = self.enemy {
            enemy.input(input_state);
        }
    }

    pub fn draw(&mut self, renderer: &mut Renderer, scene: &Scene) {
        match (&mut self.player, &mut self.enemy, &mut self.bullet) {
            (Some(ref mut player), _, _) => player.draw(renderer, scene),
            (_, Some(ref mut enemy), _) => enemy.draw(renderer, scene),
            (_, _, Some(ref mut bullet)) => bullet.draw(renderer, scene),
            _ => panic!("Incorrectly constructed object")
        }
    }

    pub fn update(&mut self, new_objects: &mut Vec<Option<GameObject>>) {
        if let Some(ref mut player) = self.player {
            player.update();

            if player.is_shooting {
                let new_bullet = player.shoots();
                new_objects.push(Some(new_bullet));
            }
        } else if let Some(ref mut enemy) = self.enemy {
            enemy.update();
        } else if let Some(ref mut bullet) = self.bullet {
            bullet.update();
        }
    }

    pub fn is_destroyed(&self) -> bool {
        match (&self.player, &self.enemy) {
            (Some(ref player), _) => player.is_destroyed,
            (_, Some(ref enemy)) => enemy.is_destroyed,
            _ => false
        }
    }

    pub fn check_collision(&self, collider: &Option<GameObject>) -> bool {
        match collider {
            Some(collider) => {
                println!("Checking collision: {:?} with {:?}", self.id, collider.id);
                true
            }
            _ => false
        }
    }
}
