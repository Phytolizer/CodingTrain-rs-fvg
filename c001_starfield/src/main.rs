use std::time::Duration;
use std::time::Instant;

use femtovg::renderer::OpenGl;
use femtovg::Canvas;
use femtovg::Color;
use glutin::event::Event;
use glutin::event::WindowEvent;
use glutin::event_loop::ControlFlow;
use glutin::ContextBuilder;
use star::Star;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

mod star;

fn main() {
    let event_loop = EventLoop::new();

    let (renderer, windowed_context) = {
        let wb = WindowBuilder::new()
            .with_title("Starfield")
            .with_inner_size(winit::dpi::LogicalSize::<f64>::new(800.0, 800.0));
        let windowed_context = ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(wb, &event_loop)
            .unwrap();
        let windowed_context = unsafe { windowed_context.make_current().unwrap() };

        let renderer = OpenGl::new(|f| windowed_context.get_proc_address(f))
            .expect("Failed to create renderer");

        (renderer, windowed_context)
    };

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
