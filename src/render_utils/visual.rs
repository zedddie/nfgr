use crate::Complex;
use crate::math::calc_buffer::MathPlot;
use crate::math::complex::DComplex;
use crate::render_utils::draw::draw_fractal;
use std::thread::sleep;
use std::time::{Duration, Instant};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

#[derive(Default)]
struct App {
    window: Option<Window>,
}
struct Plot {
    height: f64,
    width: f64,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let window = self.window.as_ref().unwrap();

                let func = |z: DComplex| z.powi(3) - DComplex::cst(Complex { rl: 1.0, im: 0.0 });

                draw_fractal(window, MathPlot { x: 4_f64, y: 4_f64 }, func);

                window.request_redraw();
            }
            _ => (),
        }
    }
}

pub fn init() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app);
}
