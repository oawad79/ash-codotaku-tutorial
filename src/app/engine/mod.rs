use anyhow::Result;
use std::{collections::HashMap, sync::Arc};
use winit::{
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes, WindowId},
};

mod renderer;
use renderer::Renderer;

pub struct Engine {
    renderers: HashMap<WindowId, Renderer>,
    windows: HashMap<WindowId, Arc<Window>>,
    primary_window_id: WindowId,
}

impl Engine {
    pub fn new(event_loop: &ActiveEventLoop) -> Result<Self> {
        let primary_window = Arc::new(event_loop.create_window(WindowAttributes::default())?);
        let primary_window_id = primary_window.id();
        let windows = HashMap::from([(primary_window_id, primary_window)]);

        let renderers = windows
            .iter()
            .map(|(id, window)| {
                let renderer = Renderer::new(window.clone()).unwrap();
                (*id, renderer)
            })
            .collect::<HashMap<_, _>>();

        Ok(Self {
            renderers,
            windows,
            primary_window_id,
        })
    }

    pub fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                if window_id == self.primary_window_id {
                    event_loop.exit();
                }
            }
            _ => {}
        }
    }
}
