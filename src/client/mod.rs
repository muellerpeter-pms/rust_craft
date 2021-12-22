use log::*;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub struct Client {
    event_loop: EventLoop<()>,
    window: Window,
}

impl Client {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("RustCraft")
            .build(&event_loop).unwrap();

        info!(target:"app::client", "created window");
        Client { window, event_loop }
    }

    pub fn run(self) {
        info!(target:"app::client", "run eventloop");
        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            let window = &self.window;
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == window.id() => *control_flow = ControlFlow::Exit,
                _ => (),
            }
        });
    }
}
