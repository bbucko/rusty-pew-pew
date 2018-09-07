use game::enemy::EnemyState;
use game::GameObject;
use game::Id;
use game::player::PlayerState;
use game::Position;
use std::collections::HashMap;
use std::num::ParseFloatError;
use std::sync::atomic::{self, AtomicUsize};

static OBJECT_COUNTER: AtomicUsize = <AtomicUsize>::new(1);

fn next_id() -> Id {
    Id::from(OBJECT_COUNTER.fetch_add(1, atomic::Ordering::SeqCst))
}

pub fn create_game_object(properties: &HashMap<String, String>) -> Result<GameObject, String> {
    let object_type = properties.get("type").expect("Unknown type").as_str();

    let x = parse_float(properties, "x")?;
    let y = parse_float(properties, "y")?;

    let position = Position::new(x, y);

    let id = next_id();

    match object_type {
        "Enemy" => Ok(GameObject { id, player: Some(PlayerState::new(position)), enemy: None }),
        "Player" => Ok(GameObject { id, player: None, enemy: Some(EnemyState::new(position)) }),
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
