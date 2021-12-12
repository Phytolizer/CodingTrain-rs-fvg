use std::time::Duration;
use std::time::Instant;

use femtovg::Canvas;
use femtovg::Color;
use femtovg::Renderer;

const SPEED: f64 = 20.0;
const SIZE: u32 = 75;

pub(crate) struct Invader {
    x: f64,
    y: f64,
    last_update: Instant,
}

impl Invader {
    pub(crate) fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            last_update: Instant::now(),
        }
    }

    pub(crate) fn update(&mut self, dt: Duration) {
        let now = Instant::now();
        if now - self.last_update > Duration::from_millis(500) {
            self.x += SPEED;
            self.last_update = now;
        }
    }

    pub(crate) fn show<R: Renderer>(&self, canvas: &mut Canvas<R>) {
        canvas.clear_rect(self.x as u32, self.y as u32, SIZE, SIZE, Color::rgb(255, 0, 0));
    }
}
