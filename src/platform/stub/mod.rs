use crate::element::Element;
use crate::mouse::*;
use crate::platform;
use crate::window::{Size, WindowImpl};

#[derive(Default)]
pub struct WindowHandle {}

struct PlatformWindow {}

impl WindowImpl for PlatformWindow {
    fn open(
        &mut self,
        dimensions: Size,
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
    fn drop(&mut self) {
        unimplemented!()
    }
}


impl MouseHandler for PlatformWindow {
    fn mouse_press(&mut self, button: MouseButton, x: isize, y: isize) {
        unimplemented!()
    }
    fn mouse_release(&mut self, button: MouseButton, x: isize, y: isize) {
        unimplemented!()
    }
    fn mouse_motion(&mut self, x: isize, y: isize) {
        unimplemented!()
    }
    fn mouse_wheel(&mut self, x: isize, y: isize, delta: f32) {
        unimplemented!()
    }
    fn mouse_enter_window(&mut self, x: isize, y: isize) {
        unimplemented!()
    }
    fn mouse_leave_window(&mut self, x: isize, y: isize) {
        unimplemented!()
    }

    fn mouse_double_click_interval(&self) -> i64 {
        unimplemented!()
    }
    fn set_cursor(&mut self, mouse: &mut Mouse, cursor: MouseCursor) {
        unimplemented!()
    }
    fn mouse_pointer_wrap(&self, x: isize, y: isize) {
        unimplemented!()
    }
    fn copy_to_clipboard(&self, buffer: &[u8]) {
        unimplemented!()
    }
    fn paste_from_clipboard(&self, buffer: &mut [u8]) -> isize {
        unimplemented!()
    }
}

pub fn create_platform_window() -> Box<WindowImpl> {
    Box::new(PlatformWindow {})
}
