use crate::event::*;
use crate::platform::PlatformWindow;
use crate::event::EventHandler;

use log::*;
use std::ffi::c_void;

pub struct Size {
    pub width: i32,
    pub height: i32,
}

pub struct Window {
    platform_window: Box<WindowImpl>,
}

impl Window {
    pub fn attach(parent: *mut c_void, dimensions: Size, title: &str, event_handler: Box<EventHandler + Send>) -> Self {
        let mut window = Window {
            platform_window: Box::new(PlatformWindow::attach(parent)),
        };
        window.platform_window.resize(dimensions);
        window.platform_window.set_title(title);
        window.platform_window.add_events_hook(event_handler);

        window
    }
}

pub trait WindowImpl {
    fn attach(parent: *mut c_void) -> PlatformWindow
    where
        Self: Sized;
    fn resize(&mut self, _size: Size) {
        info!("Window::resize()");
    }
    fn set_title(&mut self, _title: &str) {
        info!("Window::set_title()");
    }
    fn add_events_hook(&mut self, event_handler: Box<dyn EventHandler + Send>);
}
