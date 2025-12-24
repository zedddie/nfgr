use crate::Complex;
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

pub fn calculate_buffer(window: Window, math_plot_size: MathPlot) {
    let window_size = window.inner_size();
    let plot_area = PlotArea {
        width: window_size.width as u64,
        height: window_size.height as u64,
    };
    let pq_step_x = math_plot_size.x / plot_area.width as f64;
    let pq_step_y = math_plot_size.y / plot_area.height as f64;

    for py in 0..plot_area.height {
        for px in 0..plot_area.width {
            let idx = py * plot_area.width + px;
            let rl = -(math_plot_size.x / 2_f64) + (px as f64 + 0.5) * pq_step_x;
            let im = math_plot_size.y / 2_f64 - (py as f64 + 0.5) * pq_step_y;
        }
    }
}
