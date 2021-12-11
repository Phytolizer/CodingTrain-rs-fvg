use femtovg::renderer::OpenGl;
use glutin::dpi::LogicalSize;
use glutin::event_loop::EventLoop;
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;

pub fn create_renderer_and_context(
    event_loop: &EventLoop<()>,
    title: &str,
    inner_size: LogicalSize<f64>,
) -> (
    OpenGl,
    glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>,
) {
    let wb = WindowBuilder::new()
        .with_title(title)
        .with_inner_size(inner_size);
    let windowed_context = ContextBuilder::new()
        .with_vsync(true)
        .build_windowed(wb, event_loop)
        .unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };
    let renderer =
        OpenGl::new(|f| windowed_context.get_proc_address(f)).expect("Failed to create renderer");
    (renderer, windowed_context)
}
