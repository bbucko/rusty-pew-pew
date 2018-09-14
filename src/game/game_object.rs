use game::BulletState;
use game::CollisionState;
use game::EnemyState;
use game::GameObject;
use game::Id;
use game::InputState;
use game::ObjectType;
use game::PlayerState;
use game::Position;
use game::Renderer;
use game::Scene;
use game::Velocity;
use std::fmt::Debug;
use std::fmt::Error;
use std::fmt::Formatter;

impl GameObject {
    pub fn new(id: Id, position: Position, object_type: ObjectType, height: u32, width: u32) -> Self {
        let collider = Some(CollisionState { position: position.clone(), height, width });

        let mut object = GameObject { id, player: None, enemy: None, bullet: None, collider, object_type };

        match object_type {
            ObjectType::Enemy => object.enemy = Some(EnemyState::new(id, position)),
            ObjectType::Player => object.player = Some(PlayerState::new(id, position)),
            _ => panic!("unknown type: {:?}", object_type),
        }
        object
    }

    pub fn new_bullet(id: Id, position: Position, shooter_type: ObjectType, shooter_id: Id) -> Self {
        let collider = Some(CollisionState { position, height: 30, width: 16 });

        let velocity = match shooter_type {
            ObjectType::Enemy => Velocity::new(0.0, 4.0),
            ObjectType::Player => Velocity::new(0.0, -4.0),
            _ => panic!("Unknown shooter")
        };

        let bullet = Some(BulletState { position: position + Position::new(0.0, -35.0), shooter_type, shooter_id, velocity, is_destroyed: false });
        let object_type = ObjectType::Bullet;

        GameObject { id, player: None, enemy: None, bullet, collider, object_type }
    }

    pub fn handle_input(&mut self, input_state: &[InputState]) {
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
        let new_object = match (&mut self.player, &mut self.enemy, &mut self.bullet) {
            (Some(ref mut player), _, _) => player.update(),
            (_, Some(ref mut enemy), _) => enemy.update(),
            (_, _, Some(ref mut bullet)) => bullet.update(),
            _ => panic!("Incorrectly constructed or unknown object")
        };

        if new_object.is_some() { new_objects.push(new_object); }

        let position = self.position();
        match &mut self.collider {
            Some(collider) => collider.position = position,
            _ => {}
        }
    }

    fn position(&self) -> Position {
        match (&self.player, &self.enemy, &self.bullet) {
            (Some(ref player), _, _) => player.position.clone(),
            (_, Some(ref enemy), _) => enemy.position.clone(),
            (_, _, Some(ref bullet)) => bullet.position.clone(),
            _ => panic!("Incorrectly constructed or unknown object")
        }
    }

    pub fn is_destroyed(&self) -> bool {
        match (&self.player, &self.enemy, &self.bullet) {
            (Some(ref player), _, _) => player.is_destroyed,
            (_, Some(ref enemy), _) => enemy.is_destroyed,
            (_, _, Some(ref bullet)) => bullet.is_destroyed,
            _ => false
        }
    }

    pub fn destroy(&mut self) {
        match (&mut self.player, &mut self.enemy, &mut self.bullet) {
            (Some(ref mut player), _, _) => player.is_destroyed = true,
            (_, Some(ref mut enemy), _) => enemy.is_destroyed = true,
            (_, _, Some(ref mut bullet)) => bullet.is_destroyed = true,
            _ => {}
        }
    }

    fn collided_with(&mut self, collider: &mut GameObject) {
        let mut hit = false;
        if self.is_bullet() && !collider.is_bullet() {
            hit = match &self.bullet {
                Some(bullet) if !bullet.is_fired_by(collider) => {
                    println!("{:?} was hit by {:?}", collider, self);
                    true
                }
                _ => false
            }
        } else if collider.is_bullet() && !self.is_bullet() {
            hit = match &collider.bullet {
                Some(bullet) if !bullet.is_fired_by(self) => {
                    println!("{:?} was hit by {:?}", self, collider);
                    true
                }
                _ => false
            }
        }
        if hit {
            collider.destroy();
            self.destroy();
        }
    }

    pub fn check_collision(&mut self, collider: &mut GameObject) -> bool {
        let collided = match (&self.collider, &collider.collider) {
            (Some(ref a), Some(ref b)) => a.is_colliding(&b),
            _ => false
        };

        if collided {
            self.collided_with(collider);
        }

        collided
    }

    fn is_bullet(&self) -> bool { self.object_type == ObjectType::Bullet }
}

impl Debug for GameObject {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let position = self.position();
        let object_type = match (&self.player, &self.enemy, &self.bullet) {
            (Some(_), None, None) => "player",
            (None, Some(_), None) => "enemy",
            (None, None, Some(_)) => "bullet",
            _ => "unknown",
        };
        write!(f, "GameObject #{:?} of type: {} at {{ x: {}, y: {} }}", self.id, object_type, position.x, position.y);
        Ok(())
    }
}
