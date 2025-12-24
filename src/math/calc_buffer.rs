use crate::math::complex::{Complex, DComplex};
use crate::math::newton_method::calc_root;
// 1. plot buffer calculation;
// 2. viewarea & full plot size structs
// 3. colours & luminosity calculation

use winit::window::Window;

pub struct MathPlot {
    x: f64,
    y: f64,
}
struct PlotArea {
    width: u64,
    height: u64,
}

pub fn calculate_buffer<F>(window: Window, f: F, math_plot_size: MathPlot) -> Vec<u32>
where
    F: Fn(DComplex) -> DComplex + Clone,
{
    let window_size = window.inner_size();
    let plot_area = PlotArea {
        width: window_size.width as u64,
        height: window_size.height as u64,
    };
    let pq_step_x = math_plot_size.x / plot_area.width as f64;
    let pq_step_y = math_plot_size.y / plot_area.height as f64;

    let mut buf = Vec::<u32>::new();

    for py in 0..plot_area.height {
        for px in 0..plot_area.width {
            let idx = py * plot_area.width + px;
            let rl = -(math_plot_size.x / 2_f64) + (px as f64 + 0.5) * pq_step_x;
            let im = math_plot_size.y / 2_f64 - (py as f64 + 0.5) * pq_step_y;

            let conv_root = calc_root(Complex { rl, im }, &f, 100);
            apply_coloring(conv_root.0, conv_root.1, 100_usize);
            buf.insert(
                idx as usize,
                apply_coloring(conv_root.0, conv_root.1, 100_usize),
            );
            // 2. [ ] - Implement generic roots color choosing
        }
    }
    buf
}
fn apply_coloring(z: Complex, iters: usize, max_iter: usize) -> u32 {
    if iters >= max_iter {
        return 0x000000;
    }

    let angle = z.im.atan2(z.rl);
    let hue = (angle + std::f64::consts::PI) / (2.0 * std::f64::consts::PI);
    let intensity = 1.0 - (iters as f32 / max_iter as f32).powf(0.5);

    hsv_to_rgb(hue as f32, 0.8, intensity)
}

// h: 0.0-1.0, s: 0.0-1.0, v: 0.0-1.0
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> u32 {
    let i = (h * 6.0).floor();
    let f = h * 6.0 - i;
    let p = v * (1.0 - s);
    let q = v * (1.0 - f * s);
    let t = v * (1.0 - (1.0 - f) * s);

    let (r, g, b) = match i as i32 % 6 {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        _ => (v, p, q),
    };

    (((r * 255.0) as u32) << 16) | (((g * 255.0) as u32) << 8) | ((b * 255.0) as u32)
}
