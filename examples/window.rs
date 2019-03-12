#![allow(deprecated)]

extern crate rtb_rs;
extern crate winit;

fn main() {
    let mut events_loop = winit::EventsLoop::new();

    let window = winit::WindowBuilder::new()
        .with_title("winit window")
        .build(&events_loop)
        .unwrap();

    let test_string = "test_string";

    rtb_rs::Window::attach(
        unsafe { window.platform_window() } as *mut std::ffi::c_void,
        rtb_rs::Size {
            width: 640,
            height: 480,
        },
        "my window",
        move |event| {
            println!("{:?}: {}", event, test_string);
        },
    );

    events_loop.run_forever(|event| match event {
        winit::Event::WindowEvent {
            event: winit::WindowEvent::CloseRequested,
            ..
        } => winit::ControlFlow::Break,
        _ => winit::ControlFlow::Continue,
    });
}
