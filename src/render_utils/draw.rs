use softbuffer::{Context, Surface};
use std::cell::RefCell;
use std::collections::HashMap;
use std::mem;
use std::num::NonZeroU32;
use std::rc::Rc;
use winit::window::{Window, WindowId};

use crate::math::calc_buffer::{MathPlot, calculate_buffer};
use crate::math::complex::DComplex;

thread_local! {
    static GC: RefCell<Option<GraphicsContext>> = RefCell::new(None);
}

struct GraphicsContext {
    context: Context<Rc<Window>>,
    surfaces: HashMap<WindowId, (Rc<Window>, Surface<Rc<Window>, Rc<Window>>)>,
}

pub fn draw_fractal<F>(window: &Window, math_plot: MathPlot, f: F)
where
    F: Fn(DComplex) -> DComplex + Clone,
{
    GC.with(|gc| {
        let size = window.inner_size();
        let (Some(width), Some(height)) =
            (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
        else {
            return;
        };

        let mut gc_borrow = gc.borrow_mut();
        let gc = gc_borrow.get_or_insert_with(|| {
            let rc_window = unsafe { Rc::from_raw(window as *const Window) };
            let context = Context::new(rc_window.clone()).expect("Ctx failed");
            mem::forget(rc_window);
            GraphicsContext {
                context,
                surfaces: HashMap::new(),
            }
        });

        let surface = &mut gc
            .surfaces
            .entry(window.id())
            .or_insert_with(|| {
                let rc_window = unsafe { Rc::from_raw(window as *const Window) };
                let surface = Surface::new(&gc.context, rc_window.clone()).expect("Surf failed");
                mem::forget(rc_window.clone());
                (rc_window, surface)
            })
            .1;

        surface.resize(width, height).unwrap();
        let mut buffer = surface.buffer_mut().unwrap();

        calculate_buffer(
            &mut buffer,
            width.get() as usize,
            height.get() as usize,
            f,
            math_plot,
        );

        buffer.present().unwrap();
    });
}
