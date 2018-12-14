use crate::element::Element;
use crate::platform;
use crate::window::{WindowDimensions, WindowImpl};

#[derive(Default)]
pub struct WindowHandle {}

struct PlatformWindow {}

impl WindowImpl for PlatformWindow {
    fn open(
        &mut self,
        dimensions: WindowDimensions,
        title: &str,
        parent: Option<platform::WindowHandle>,
    ) {
        unimplemented!()
    }
    fn draw(&mut self, force_redraw: bool) -> bool {
        unimplemented!()
    }
    fn focus_element(&mut self, element: &mut Element) {
        unimplemented!()
    }
    fn lock(&mut self) {
        unimplemented!()
    }
    fn unlock(&mut self) {
        unimplemented!()
    }
    fn reinit(&mut self) {
        unimplemented!()
    }
}

impl Drop for PlatformWindow {
    fn drop(&mut self) {}
}

pub fn create_platform_window() -> Box<WindowImpl> {
    Box::new(PlatformWindow {})
}
