use super::sdl2::keyboard::KeyboardState;
use super::sdl2::keyboard::Scancode;
use game::InputHandler;
use game::InputState;
use sdl::sdl2::event::Event;
use sdl::SDLEngine;
use sdl::SDLInputHandler;

impl InputHandler for SDLInputHandler {
    fn capture(&mut self) -> Vec<InputState> {
        let mut input = Vec::new();

        for event in self.event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                return vec![InputState::Quit];
            };
        }

        let state = self.event_pump.keyboard_state();

        if SDLInputHandler::is_pressed(&state, &[Scancode::Up, Scancode::W]) {
            input.push(InputState::Up)
        }
        if SDLInputHandler::is_pressed(&state, &[Scancode::Down, Scancode::S]) {
            input.push(InputState::Down)
        }
        if SDLInputHandler::is_pressed(&state, &[Scancode::Left, Scancode::A]) {
            input.push(InputState::Left)
        }
        if SDLInputHandler::is_pressed(&state, &[Scancode::Right, Scancode::D]) {
            input.push(InputState::Right)
        }
        if SDLInputHandler::is_pressed(&state, &[Scancode::Space]) {
            input.push(InputState::Shoot)
        }
        input
    }
}

impl SDLInputHandler {
    pub fn new(sdl: &SDLEngine) -> Self {
        let event_pump = sdl.context.event_pump().unwrap();
        Self { event_pump }
    }

    fn is_pressed(state: &KeyboardState, scancodes: &[Scancode]) -> bool {
        for scancode in scancodes {
            if state.is_scancode_pressed(*scancode) {
                return true;
            }
        }
        false
    }
}
