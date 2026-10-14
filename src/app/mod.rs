use winit::application::ApplicationHandler;

mod engine;
use engine::Engine;

#[derive(Default)]
pub struct App {
    engine: Option<Engine>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.engine = Some(Engine::new(event_loop).unwrap());
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
    }
}
