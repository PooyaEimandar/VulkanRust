use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    // create event loop
    let event_loop = EventLoop::new();

    // create window
    let window = WindowBuilder::new()
        .with_title("Vulkan Rust: 2_window")
        .build(&event_loop)
        .unwrap();

    // run event loop which handles window's events
    event_loop.run(move |event, _eloop_win_target, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => (),
        }
    });
}
