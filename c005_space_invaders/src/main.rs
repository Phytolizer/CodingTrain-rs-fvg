use std::time::Duration;
use std::time::Instant;

use femtovg::Canvas;
use femtovg::Color;
use glutin::dpi::LogicalSize;
use glutin::event::ElementState;
use glutin::event::Event;
use glutin::event::VirtualKeyCode;
use glutin::event::WindowEvent;
use glutin::event_loop::ControlFlow;
use glutin::event_loop::EventLoop;
use helpers::graphics::create_renderer_and_context;
use invader::Invader;
use ship::Ship;

mod invader;
mod ship;

const INVADER_SPACING: f64 = 80.0;
const INVADER_OFFSET: f64 = 50.0;

fn main() {
    let event_loop = EventLoop::new();
    let (renderer, windowed_context) = create_renderer_and_context(
        &event_loop,
        "Space Invaders",
        LogicalSize::new(800.0, 600.0),
    );
    let mut canvas = Canvas::new(renderer).expect("Failed to create canvas");

    let mut ship = Ship::new(windowed_context.window());
    let mut invaders = (0..8)
        .map(|i| Invader::new(i as f64 * INVADER_SPACING + INVADER_OFFSET, INVADER_OFFSET))
        .collect::<Vec<_>>();

    let mut frame_start = Instant::now();
    let mut dt = Duration::from_secs(0);

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
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(key) = input.virtual_keycode {
                        match key {
                            VirtualKeyCode::Escape => {
                                *control_flow = ControlFlow::Exit;
                            }
                            VirtualKeyCode::Left => match input.state {
                                ElementState::Pressed => {
                                    ship.set_left(true);
                                }
                                ElementState::Released => {
                                    ship.set_left(false);
                                }
                            },
                            VirtualKeyCode::Right => match input.state {
                                ElementState::Pressed => {
                                    ship.set_right(true);
                                }
                                ElementState::Released => {
                                    ship.set_right(false);
                                }
                            },
                            _ => {}
                        }
                    }
                }
                _ => {}
            },
            Event::RedrawRequested(_) => {
                let dpi_factor = window.scale_factor();
                let size = window.inner_size();

                canvas.set_size(size.width, size.height, dpi_factor as f32);
                canvas.clear_rect(0, 0, size.width, size.height, Color::rgb(0, 0, 0));

                ship.update(window, dt);
                ship.draw(window, &mut canvas);

                for invader in invaders.iter_mut() {
                    invader.update(dt);
                    invader.show(&mut canvas);
                }

                canvas.flush();
                windowed_context.swap_buffers().unwrap();

                let now = Instant::now();
                dt = now - frame_start;
                frame_start = now;
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}
