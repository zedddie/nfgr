use crate::Complex;
// 1. plot buffer calculation;
// 2. viewarea & full plot size structs
// 3. colours & luminosity calculation

use winit::window::Window;

struct ViewArea {
    width: u32,
    height: u32,
}
struct PlotArea {
    width: u32,
    height: u32,
}

// will return buffer filled with pixels twice as large as current user viewarea
pub fn calculate_buffer(window: Window, math_plot_size: u32) {
    let window_size = window.inner_size();
    let plot_area = PlotArea {
        width: window_size.width * 2,
        height: window_size.height * 2,
    };
}
