use std::cmp::max;
use std::time::Duration;

use femtovg::Canvas;
use femtovg::Color;
use femtovg::Paint;
use femtovg::Path;
use femtovg::Renderer;
use glutin::window::Window;
use helpers::math::map;
use rand::Rng;

pub(crate) struct Star {
    x: f64,
    y: f64,
    z: f64,
    pz: f64,
}

impl Star {
    pub(crate) fn new(window: &Window) -> Self {
        let mut rng = rand::thread_rng();
        let inner_size = window.inner_size();
        let x = rng.gen_range(-(inner_size.width as f64)..inner_size.width as f64);
        let y = rng.gen_range(-(inner_size.height as f64)..inner_size.height as f64);
        let z = rng.gen_range(-(inner_size.width as f64)..inner_size.width as f64);
        let pz = z;
        Star { x, y, z, pz }
    }

    pub(crate) fn update(&mut self, window: &Window, delta: Duration) {
        let inner_size = window.inner_size();
        self.pz = self.z;
        self.z -= delta.as_secs_f64() * inner_size.width as f64;
        if self.z < 1.0 {
            let mut rng = rand::thread_rng();
            self.x = rng.gen_range(-(inner_size.width as f64)..inner_size.width as f64);
            self.y = rng.gen_range(-(inner_size.height as f64)..inner_size.height as f64);
            self.z = rng.gen_range(inner_size.width as f64..(inner_size.width as f64) * 2.0);
            self.pz = self.z;
        }
    }

    pub(crate) fn show<R>(&self, window: &Window, canvas: &mut Canvas<R>)
    where
        R: Renderer,
    {
        let inner_size = window.inner_size();
        let sx = map(self.x / self.z, 0.0, 1.0, 0.0, inner_size.width as f64)
            + (inner_size.width as f64) / 2.0;
        let sy = map(self.y / self.z, 0.0, 1.0, 0.0, inner_size.height as f64)
            + (inner_size.height as f64) / 2.0;
        let r = {
            let mut r = map(self.z, 0.0, inner_size.width as f64, 8.0, 0.0);
            if r < 0.0 {
                r = 0.0;
            }
            r
        };

        let bg = Paint::color(Color::rgb(255, 255, 255));
        let mut path = Path::new();
        path.circle(sx as f32, sy as f32, r as f32);
        canvas.fill_path(&mut path, bg);
    }
}
