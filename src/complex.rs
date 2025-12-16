#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Complex {
    rl: f64,
    im: f64,
}

impl Complex {
    pub fn new(rl: f64, im: f64) -> Self {
        Self { rl, im }
    }
}
