use std::time::Duration;
use std::time::Instant;

use femtovg::Canvas;
use femtovg::Color;
use glutin::dpi::LogicalSize;
use glutin::event::Event;
use glutin::event::WindowEvent;
use glutin::event_loop::ControlFlow;
use helpers::graphics::create_renderer_and_context;
use star::Star;
use winit::event_loop::EventLoop;

mod star;

fn main() {
    let event_loop = EventLoop::new();

    let (renderer, windowed_context) =
        create_renderer_and_context(&event_loop, "Starfield", LogicalSize::new(800.0, 800.0));

    let mut canvas = Canvas::new(renderer).expect("Failed to create canvas");

    let mut stars = Vec::with_capacity(1000);
    for _ in 0..1000 {
        stars.push(Star::new(windowed_context.window()));
    }

    let mut frame_start = Instant::now();
    let mut delta = Duration::from_secs(0);

    event_loop.run(move |event, _, control_flow| {
        let window = windowed_context.window();
        *control_flow = ControlFlow::Poll;

        match event {
            Event::LoopDestroyed => {}
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    windowed_context.resize(*physical_size);
                }
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            },
            Event::RedrawRequested(_) => {
                let dpi_factor = window.scale_factor();
                let size = window.inner_size();

                for star in stars.iter_mut() {
                    star.update(window, delta);
                }

                canvas.set_size(size.width, size.height, dpi_factor as f32);
                canvas.clear_rect(0, 0, size.width, size.height, Color::rgb(0, 0, 0));

                for star in stars.iter() {
                    star.show(window, &mut canvas);
                }

                canvas.flush();
                windowed_context.swap_buffers().unwrap();
                let frame_end = Instant::now();
                delta = frame_end - frame_start;
                frame_start = frame_end;
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}
