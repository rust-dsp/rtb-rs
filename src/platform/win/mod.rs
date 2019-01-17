
use crate::event::Event;
use crate::window::WindowImpl;
use std::ffi::c_void;

pub struct PlatformWindow;

impl WindowImpl for PlatformWindow {
    fn attach(_parent: *mut c_void) -> Self {
        PlatformWindow {}
    }

    fn add_events_hook(&mut self, _events: Box<Fn(Event)>) {}
}
