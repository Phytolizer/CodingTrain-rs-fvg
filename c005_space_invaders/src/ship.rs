use std::time::Duration;

use femtovg::Canvas;
use femtovg::Color;
use femtovg::Paint;
use femtovg::Path;
use femtovg::Renderer;
use glutin::window::Window;

pub(crate) struct Ship {
    x: f64,
    dx: f64,
    left: bool,
    right: bool,
}

const SHIP_SPEED: f64 = 250.0;

impl Ship {
    pub(crate) fn new(window: &Window) -> Self {
        let width = window.inner_size().width;
        Self {
            x: width as f64 / 2.0,
            dx: 0.0,
            left: false,
            right: false,
        }
    }

    pub(crate) fn update(&mut self, window: &Window, dt: Duration) {
        let width = window.inner_size().width;
        self.x += self.dx * dt.as_secs_f64() * SHIP_SPEED;
        if self.x < 0.0 {
            self.x = 0.0;
        } else if self.x > width as f64 {
            self.x = width as f64;
        }
    }

    pub(crate) fn draw<R: Renderer>(&self, window: &Window, canvas: &mut Canvas<R>) {
        let paint = Paint::color(Color::rgb(255, 255, 255));
        let mut path = Path::new();
        let bottom = window.inner_size().height as f64;
        path.move_to((self.x - 50.0) as f32, bottom as f32);
        path.line_to((self.x + 50.0) as f32, bottom as f32);
        path.line_to((self.x) as f32, bottom as f32 - 80.0);
        path.close();
        canvas.fill_path(&mut path, paint);
    }

    pub(crate) fn set_left(&mut self, left: bool) {
        self.left = left;
        if left {
            self.dx = -1.0;
        } else if self.right {
            self.dx = 1.0;
        } else {
            self.dx = 0.0;
        }
    }

    pub(crate) fn set_right(&mut self, right: bool) {
        self.right = right;
        if right {
            self.dx = 1.0;
        } else if self.left {
            self.dx = -1.0;
        } else {
            self.dx = 0.0;
        }
    }
}
