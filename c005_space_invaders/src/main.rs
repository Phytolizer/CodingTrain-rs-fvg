use femtovg::Canvas;
use femtovg::Color;
use glutin::dpi::LogicalSize;
use glutin::event::Event;
use glutin::event::WindowEvent;
use glutin::event_loop::ControlFlow;
use glutin::event_loop::EventLoop;
use helpers::graphics::create_renderer_and_context;

fn main() {
    let event_loop = EventLoop::new();
    let (renderer, windowed_context) = create_renderer_and_context(
        &event_loop,
        "Space Invaders",
        LogicalSize::new(800.0, 600.0),
    );
    let mut canvas = Canvas::new(renderer).expect("Failed to create canvas");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        let window = windowed_context.window();
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

                canvas.set_size(size.width, size.height, dpi_factor as f32);
                canvas.clear_rect(0, 0, size.width, size.height, Color::rgb(0, 0, 0));

                canvas.flush();
                windowed_context.swap_buffers().unwrap();
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}
