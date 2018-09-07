use game::enemy::Enemy;
use game::Entity;
use game::Id;
use game::Position;
use std::collections::HashMap;
use std::num::ParseFloatError;
use std::sync::atomic::{self, AtomicUsize};
use game::player::Player;

static OBJECT_COUNTER: AtomicUsize = <AtomicUsize>::new(1);

fn next_id() -> Id {
    Id::from(OBJECT_COUNTER.fetch_add(1, atomic::Ordering::SeqCst))
}

pub fn create(properties: &HashMap<String, String>) -> Result<Box<Entity>, String> {
    let object_type = properties.get("type").expect("Unknown type").as_str();

    let x = parse_float(properties, "x")?;
    let y = parse_float(properties, "y")?;

    match object_type {
        "Enemy" => Ok(Box::new(Enemy::new(next_id(), Position::new(x, y)))),
        "Player" => Ok(Box::new(Player::new(next_id(), Position::new(x, y)))),
        _ => panic!("unknown type: {:?}", object_type),
    }
}

fn parse_float(properties: &HashMap<String, String>, attribute_name: &str) -> Result<f32, String> {
    properties
        .get(attribute_name)
        .expect(&format!("Missing: {:?}", attribute_name))
        .parse()
        .map_err(|e: ParseFloatError| e.to_string())
}
