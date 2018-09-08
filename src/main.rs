extern crate base64;
extern crate cgmath;
extern crate core;
#[macro_use]
extern crate lazy_static;

use game::GameObject;
use game::GameState;
use game::InputHandler;
use game::InputState;
use game::Renderer;
use game::states::BulletState;
use helpers::parsers;
use std::thread;
use std::time::Duration;
use std::time::SystemTime;

mod game;
mod helpers;
mod sdl;


const FPS: u8 = 60;

lazy_static! {
    static ref DELAY: Duration = Duration::new(0, 1000000000 / FPS as u32);
}


pub fn main() {
    println!("Starting up");
    let (mut scene, texture_wrappers) = parsers::map_file::parse("assets/map1.tmx");
    let scene = &mut scene;

    let sdl_context = sdl::SDLEngine::init();

    let mut input_handler = sdl::SDLInputHandler::new(&sdl_context);

    let (canvas, texture_creator) = sdl::SDLRenderer::init(&sdl_context);
    let texture_manager = sdl::TextureManager::new(&texture_creator);
    let renderer = &mut sdl::SDLRenderer::new(canvas, texture_manager, texture_wrappers);

    loop {
        let frame_start = SystemTime::now();
        let inputs = input_handler.capture();

        if inputs.contains(&InputState::Quit) {
            break;
        }

        handle_input(scene, &inputs);

        update(scene);

        draw(scene, renderer);

        frame_limiter(frame_start);
    }
    println!("Shutting down");
}

fn frame_limiter(frame_start: SystemTime) {
    let now = SystemTime::now();
    let duration = now.duration_since(frame_start).unwrap();
    if duration.le(&DELAY) {
        thread::sleep(*DELAY - duration);
    }
}

fn draw(scene: &mut GameState, renderer: &mut Renderer) {
    renderer.render(&mut scene.game_objects);
}


fn update(scene: &mut GameState) {
    let mut new_objects = Vec::new();
    {
        for game_object in &mut scene.game_objects {
            if let Some(game_object) = game_object {
                if let Some(ref mut player) = game_object.player {
                    player.update();

                    if player.is_shooting {
                        let new_bullet = GameObject { player: None, enemy: None, bullet: Some(BulletState::player_shoots(&player)), id: 111 };
                        new_objects.push(Some(new_bullet));
                        player.is_shooting = false;
                    }

                    if player.is_destroyed {}
                }

                if let Some(ref mut enemy) = game_object.enemy {
                    enemy.update();
                }
            }
        }

        for outer in &scene.game_objects {
            if let Some(game_object) = outer {
                for inner in &scene.game_objects {
                    if let Some(inner) = inner {
                        if inner.id != game_object.id {
                            game_object.check_collision(&inner);
                        }
                    }
                }
            }
        }
    }

    scene.game_objects.extend(new_objects);
    scene.game_objects.retain(|game_object| {
        if let Some(ref game_object) = game_object {
            return !game_object.is_destroyed();
        }
        return true;
    });
}

fn handle_input(scene: &mut GameState, input_state: &Vec<InputState>) {
    for game_object in &mut scene.game_objects {
        if let Some(game_object) = game_object {
            game_object.input(input_state);
        }
    }
}
