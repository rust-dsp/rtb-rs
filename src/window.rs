use crate::element::Element;
use crate::mouse::MouseHandler;
use crate::platform;
use log::*;

pub struct WindowDimensions {
    pub width: usize,
    pub height: usize,
    pub x: usize,
    pub y: usize,
}

pub struct Window {
    platform_window: Box<WindowImpl>,
}

impl Window {
    pub fn open(dimensions: WindowDimensions, title: &str) -> Self {
        info!("Window::open()");
        Self::open_under(None, dimensions, title)
    }
    pub fn open_under(
        parent: Option<platform::WindowHandle>,
        dimensions: WindowDimensions,
        title: &str,
    ) -> Self {
        info!("Window::open_under()");
        let mut platform_window = platform::create_platform_window();
        platform_window.open(dimensions, title, parent);
        Window { platform_window }
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
    /// Create a window and display it
    fn open(
        &mut self,
        dimensions: WindowDimensions,
        title: &str,
        parent: Option<platform::WindowHandle>,
    );

    /// (Re)draw the window.
    fn draw(&mut self, force_redraw: bool) -> bool;

    fn focus_element(&mut self, element: &mut Element);
    fn lock(&mut self);
    fn unlock(&mut self);
    fn reinit(&mut self);
}
