use crate::math::complex::{Complex, DComplex};
use crate::math::newton_method::calc_root;
use softbuffer::Buffer;
// 1. plot buffer calculation;
// 2. viewarea & full plot size structs
// 3. colours & luminosity calculation

use winit::window::Window;

pub struct MathPlot {
    pub x: f64,
    pub y: f64,
}
#[derive(Clone)]
pub struct PlotArea {
    pub width: u64,
    pub height: u64,
}

pub fn calculate_buffer<F>(
    buf: &mut [u32],
    width: usize,
    height: usize,
    f: F,
    math_plot_size: MathPlot,
) where
    F: Fn(DComplex) -> DComplex + Clone,
{
    // let plot_area = PlotArea {
    //     width: window_size.width as u64,
    //     height: window_size.height as u64,
    // };
    let pq_step_x = math_plot_size.x / width as f64;
    let pq_step_y = math_plot_size.y / height as f64;

    for py in 0..height {
        for px in 0..width {
            let idx = py * width + px;
            let rl = -(math_plot_size.x / 2_f64) + (px as f64 + 0.5) * pq_step_x;
            let im = math_plot_size.y / 2_f64 - (py as f64 + 0.5) * pq_step_y;

            let conv_root = calc_root(Complex { rl, im }, &f, 300);
            apply_coloring(conv_root.0, conv_root.1, 100_usize);
            buf[idx as usize] = apply_coloring(conv_root.0, conv_root.1, 100_usize);
            // 2. [ ] - Implement generic roots color choosing
        }
    }
}
// root expectant
struct RootE {
    root: Complex,
    counter: u32,
}
pub fn precalc_root_colours<F>(
    width: &usize,
    height: &usize,
    f: F,
    math_plot_size: &MathPlot,
) -> Vec<Complex>
where
    F: Fn(DComplex) -> DComplex + Clone,
{
    let mut root_vec = Vec::<RootE>::new();

    let pq_step_x = (math_plot_size.x / *width as f64) * 10_f64;
    let pq_step_y = (math_plot_size.y / *height as f64) * 10_f64;

    for py in 0..*height / 10 {
        for px in 0..*width / 10 {
            let rl = -(math_plot_size.x / 2_f64) + (px as f64 + 0.5) * pq_step_x;
            let im = math_plot_size.y / 2_f64 - (py as f64 + 0.5) * pq_step_y;

            let conv_root = calc_root(Complex { rl, im }, &f, 300);
            match root_vec.iter_mut().find(|root_e| {
                (root_e.root.im - conv_root.0.im).abs() < 1e-4
                    && (root_e.root.rl - conv_root.0.rl).abs() < 1e-4
            }) {
                None => root_vec.push(RootE {
                    root: conv_root.0,
                    counter: 1,
                }),
                root => root.unwrap().counter += 1,
            }
        }
    }
    let roots: Vec<Complex> = root_vec
        .iter()
        .filter(|e| e.counter >= 20)
        .map(|r| r.root)
        .collect();
    roots
}
fn apply_coloring(z: Complex, iters: usize, max_iter: usize) -> u32 {
    if iters >= max_iter {
        return 0x000000;
    }
    42

    // let angle = z.im.atan2(z.rl);
    // let hue = (angle + std::f64::consts::PI) / (2.0 * std::f64::consts::PI);
    // let intensity = 1.0 - (iters as f32 / max_iter as f32).powf(0.5);
    //
    // hsv_to_rgb(hue as f32, 0.8, intensity)
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
