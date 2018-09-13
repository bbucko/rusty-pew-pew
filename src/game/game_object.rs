use game::GameObject;
use game::InputState;
use game::Renderer;
use game::Scene;
use game::states::BulletState;

impl GameObject {
    pub fn input(&mut self, input_state: &[InputState]) {
        if let Some(ref mut player) = self.player {
            player.input(input_state);
        }

        if let Some(ref mut enemy) = self.enemy {
            enemy.input(input_state);

            let _new_bullet = BulletState::enemy_bullet(enemy);
//            println!("Enemy shoots: {:?}", new_bullet);
        }
    }

    pub fn draw(&mut self, renderer: &mut Renderer, scene: &Scene) {
        if let Some(ref mut player) = self.player {
            player.draw(renderer, scene);
        }

        if let Some(ref mut enemy) = self.enemy {
            enemy.draw(renderer, scene);
        }

        if let Some(ref mut bullet) = self.bullet {
            bullet.draw(renderer, scene);
        }
    }

    pub fn is_destroyed(&self) -> bool {
        if let Some(ref player) = self.player {
            return player.is_destroyed;
        }

        if let Some(ref enemy) = self.enemy {
            return enemy.is_destroyed;
        }

        false
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
