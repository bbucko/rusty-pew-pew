use game::enemy::Enemy;
use game::GameObject;
use game::Pos;
use std::collections::HashMap;

pub fn create(properties: &HashMap<String, String>) -> Box<GameObject> {
    let object_type = properties.get("type").expect("Unknown type").as_str();
    match object_type {
        "Enemy" => Box::new(Enemy::new(Pos::new(120.0, 0.0))),
        _ => panic!("unknown type: {:?}", object_type)
    }
}
