use game::Engine;
use game::InputHandler;
use game::Renderer;
use game::Scene;

impl<T, U> Engine<T, U>
    where
        T: Renderer,
        U: InputHandler,
{
    pub fn new(video: T, input_handler: U, scene: Scene) -> Self {
        Engine {
            running: true,
            video,
            input_handler,
            scene,
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn handle_events(&mut self) {
        match self.input_handler.events() {
            Some(1) => self.running = false,
            Some(x) => self.scene.input(&x),
            None => ()
        }
    }

    pub fn update(&mut self) {
        self.scene.update();
    }

    pub fn render(&mut self) {
        self.video.render(&mut self.scene);
    }
}
