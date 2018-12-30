use crate::element::Element;
use crate::platform::PlatformWindow;
use crate::mouse::MouseHandler;

use std::ffi::c_void;
use log::*;

pub struct Size {
    pub width: i32,
    pub height: i32,
}

pub struct Window {
    platform_window: Box<WindowImpl>,
}

impl Window {
    pub fn attach(parent: *mut c_void, dimensions: Size, title: &str) -> Self {
        let mut window = Window {
            platform_window: Box::new(PlatformWindow::attach(parent))
        };
        window.platform_window.resize(dimensions);
        window.platform_window.set_title(title);

        window
    }
    pub fn close(self) {
        info!("Window::close()");
        //TODO: cleanup
        //drop
    }
    pub fn draw(&mut self, force_redraw: bool) -> bool {
        self.platform_window.draw(force_redraw)
    }
    pub fn focus_element(&mut self, element: &mut Element) {
        self.platform_window.focus_element(element);
    }
    pub fn lock(&mut self) {
        self.platform_window.lock();
    }
    pub fn unlock(&mut self) {
        self.platform_window.unlock();
    }
    pub fn reinit(&mut self) {
        unimplemented!();
    }
}

pub trait WindowImpl: Drop + MouseHandler {
    fn attach(parent: *mut c_void) -> PlatformWindow where Self: Sized;
    fn resize(&mut self, _size: Size) {
        info!("Window::resize()");
    }
    fn set_title(&mut self, _title: &str) {
        info!("Window::set_title()");
    }
    fn draw(&mut self, _force_redraw: bool) -> bool { unimplemented!() }
    fn focus_element(&mut self, _element: &mut Element) { unimplemented!() }
    fn lock(&mut self) { unimplemented!() }
    fn unlock(&mut self) { unimplemented!() }
    fn reinit(&mut self) { unimplemented!() }
}
