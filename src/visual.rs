use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::EventLoop;
use winit::event_loop::{ActiveEventLoop, ControlFlow};
use winit::window::{Window, WindowId};

// look into https://docs.rs/winit/latest/winit/;
// https://docs.rs/glutin/latest/glutin/; (opengl context)
#[derive(Default)]
struct App {
    window: Option<Window>,
}
pub fn init() {
    let event_loop = EventLoop::new();
}
