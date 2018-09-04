use game::enemy::Enemy;
use game::GameObject;
use game::Id;
use game::Pos;
use std::collections::HashMap;
use std::sync::atomic::{self, AtomicUsize};

static OBJECT_COUNTER: AtomicUsize = <AtomicUsize>::new(1);

fn next_id() -> Id {
    Id::from(OBJECT_COUNTER.fetch_add(1, atomic::Ordering::SeqCst))
}

pub fn create(properties: &HashMap<String, String>) -> Box<GameObject> {
    let object_type = properties.get("type").expect("Unknown type").as_str();
    let _x = properties.get("x").map_or(0, |s| s.parse().unwrap());
    let _y = properties.get("y").map_or(0, |s| s.parse().unwrap());
    match object_type {
        "Enemy" => Box::new(Enemy::new(next_id(), Pos::new(120.0, 0.0))),
        _ => panic!("unknown type: {:?}", object_type),
    }
}
