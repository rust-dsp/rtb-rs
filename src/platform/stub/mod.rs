use crate::platform;
use crate::window::{Size};

struct PlatformWindow {}

impl WindowImpl for PlatformWindow {
    fn attach(parent: *mut c_void) -> PlatformWindow {
        unimplemented!()
    }

    fn add_events_hook(&mut self, events: Box<Fn(crate::event::Event)>) {
        unimplemented!()
    }

    fn resize(&mut self, size: Size) {
        unimplemented!()
    }

    fn set_title(&mut self, title: &str) {
        unimplemented!()
    }
}

impl Drop for PlatformWindow {
    fn drop(&mut self) {
        unimplemented!()
    }
}
