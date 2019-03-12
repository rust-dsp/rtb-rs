use crate::event::*;
use crate::platform::PlatformWindow;
use std::ffi::c_void;

pub struct Size {
    pub width: i32,
    pub height: i32,
}

pub struct Window {
    platform_window: Box<WindowImpl>,
}

impl Window {
    pub fn attach<F: 'static>(parent: *mut c_void, events: F) -> Self
    where
        F: Fn(Event),
    {
        let mut window = Window {
            platform_window: Box::new(PlatformWindow::attach(parent)),
        };
        window.platform_window.add_events_hook(Box::new(events));

        window
    }
}

pub trait WindowImpl {
    fn attach(parent: *mut c_void) -> PlatformWindow
    where
        Self: Sized;
    fn resize(&mut self, _size: Size) {
        unimplemented!();
    }
    fn set_title(&mut self, _title: &str) {
        unimplemented!();
    }
    fn add_events_hook(&mut self, events: Box<Fn(Event)>);
}
