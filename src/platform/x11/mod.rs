use log::*;

use crate::window::WindowImpl;
use crate::window::WindowDimensions;
use crate::element::Element;
use crate::platform;
use crate::mouse::{Mouse, MouseHandler, MouseButton, MouseCursor};
use std::borrow::Borrow;

mod x_handle;

pub struct WindowHandle {
    id: u32,
}

impl WindowHandle {
    pub fn new(id: u32) -> Self {
        Self {
            id
        }
    }
}

// Create the X11 structure, without actually opening the window.
// TODO: Set up X11 connection, etc.
pub fn create_platform_window() -> Box<WindowImpl> {
    info!("X11::create_platform_window()");
    Box::new(X11::new())
}

pub struct X11 {
    x_handle: x_handle::XHandle,
}

impl X11 {
    fn new() -> Self {
        Self {
            x_handle: x_handle::XHandle::new(),
        }
    }
}

// TODO: remove this lint
#[allow(unused_variables)]
impl WindowImpl for X11 {
    fn open(&mut self, dimensions: WindowDimensions, title: &str, parent: Option<platform::WindowHandle>) {
        info!("WindowImpl<X11>::open()");

        if parent.is_none() {
            info!("No parent...?");
        }
        let parent_id = parent.unwrap().id; // TODO: error handle instead of unwrap
        info!("Parent id: {}", parent_id);

        // Create a draw context
        // TODO: save the draw context for later
        let conn = self.x_handle.conn();
        let setup = conn.get_setup();
        let screen = setup.roots().nth(self.x_handle.screen_num() as usize).unwrap();
        let draw_context = conn.generate_id();
        xcb::create_gc(conn.borrow(), draw_context, parent_id, &[
            (xcb::GC_FOREGROUND, screen.white_pixel()),
            (xcb::GC_GRAPHICS_EXPOSURES, 0),
        ]);

        // Create the actual window
        // TODO: save this handle for later as well
        let window_handle = conn.generate_id();
        xcb::create_window(&conn,
                           xcb::COPY_FROM_PARENT as u8,
                           window_handle,
                           parent_id,
                           0, // TODO: save this somewhere instead of magic number
                           0, // TODO: save this somewhere instead of magic number
                           1000, // TODO: save this somewhere instead of magic number
                           1000, // TODO: save this somewhere instead of magic number
                           0,
                           xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,
                           screen.root_visual(), &[
                (xcb::CW_BACK_PIXEL, screen.black_pixel()),
                (xcb::CW_EVENT_MASK,
                 xcb::EVENT_MASK_EXPOSURE |
                     xcb::EVENT_MASK_BUTTON_PRESS |
                     xcb::EVENT_MASK_BUTTON_RELEASE |
                     xcb::EVENT_MASK_BUTTON_1_MOTION
                ),
            ]
        );
        xcb::map_window(&conn, window_handle);
        conn.flush();
    }

    fn draw(&mut self, force_redraw: bool) -> bool {
        unimplemented!();
    }

    fn focus_element(&mut self, element: &mut Element) {
        unimplemented!();
    }

    fn lock(&mut self) {
        unimplemented!();
    }

    fn unlock(&mut self) {
        unimplemented!();
    }

    fn reinit(&mut self) {
        unimplemented!();
    }
}

impl Drop for X11 {
    fn drop(&mut self) {
        // TODO: actually drop
        // (removed unimplemented!() call so the plugin doesn't crash)
        info!("WindowImpl<X11>::drop()");
    }
}

// TODO: remove this lint
#[allow(unused_variables)]
impl MouseHandler for X11 {
    fn mouse_press(&mut self, button: MouseButton, x: isize, y: isize) {
        unimplemented!();
    }

    fn mouse_release(&mut self, button: MouseButton, x: isize, y: isize) {
        unimplemented!();
    }

    fn mouse_motion(&mut self, x: isize, y: isize) {
        unimplemented!();
    }

    fn mouse_wheel(&mut self, x: isize, y: isize, delta: f32) {
        unimplemented!();
    }

    fn mouse_enter_window(&mut self, x: isize, y: isize) {
        unimplemented!();
    }

    fn mouse_leave_window(&mut self, x: isize, y: isize) {
        unimplemented!();
    }

    fn mouse_double_click_interval(&self) -> i64 {
        unimplemented!();
    }

    fn set_cursor(&mut self, mouse: &mut Mouse, cursor: MouseCursor) {
        unimplemented!();
    }

    fn mouse_pointer_wrap(&self, x: isize, y: isize) {
        unimplemented!();
    }

    fn copy_to_clipboard(&self, buffer: &[u8]) {
        unimplemented!();
    }

    fn paste_from_clipboard(&self, buffer: &mut [u8]) -> isize {
        unimplemented!();
    }
}