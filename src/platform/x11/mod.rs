use log::*;

use crate::element::Element;
use crate::mouse::{Mouse, MouseButton, MouseCursor, MouseHandler};
use crate::platform;
use crate::window::WindowDimensions;
use crate::window::WindowImpl;
use std::borrow::Borrow;

mod x_handle;

pub struct WindowHandle {
    id: u32,
}

impl WindowHandle {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}

pub fn create_platform_window() -> Box<WindowImpl> {
    info!("X11::create_platform_window()");

    // Create an X11 handle
    let x_handle = x_handle::XHandle::new();

    Box::new(X11 {
        x_handle,
        draw_context: None,
        window_handle: None,
        window_dimensions: None,
    })
}

pub struct X11 {
    x_handle: x_handle::XHandle,
    draw_context: Option<u32>,
    window_handle: Option<u32>,
    window_dimensions: Option<WindowDimensions>,
}

impl X11 {}

// TODO: remove this lint
#[allow(unused_variables)]
impl WindowImpl for X11 {
    fn open(
        &mut self,
        dimensions: WindowDimensions,
        title: &str,
        parent: Option<platform::WindowHandle>,
    ) {
        info!("WindowImpl<X11>::open()");

        if parent.is_none() {
            info!("No parent...?");
            // TODO: handle this correctly. Just create a "root" window
            // instead of trying to attach it to parent.
        }
        let parent_id = parent.unwrap().id; // TODO: error handle instead of unwrap
        info!("Parent id: {}", parent_id);

        // Create a draw context
        let conn = self.x_handle.conn();
        let setup = conn.get_setup();
        let screen = setup
            .roots()
            .nth(self.x_handle.screen_num() as usize)
            .unwrap();
        let draw_context = conn.generate_id();
        xcb::create_gc(
            conn.borrow(),
            draw_context,
            parent_id,
            &[
                (xcb::GC_FOREGROUND, screen.white_pixel()),
                (xcb::GC_GRAPHICS_EXPOSURES, 0),
            ],
        );

        // Create the actual window
        let window_handle = conn.generate_id();
        xcb::create_window(
            &conn,
            xcb::COPY_FROM_PARENT as u8,
            window_handle,
            parent_id,
            dimensions.x as i16,
            dimensions.y as i16,
            dimensions.width as u16,
            dimensions.height as u16,
            0,
            xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,
            screen.root_visual(),
            &[
                (xcb::CW_BACK_PIXEL, screen.black_pixel()),
                (
                    xcb::CW_EVENT_MASK,
                    xcb::EVENT_MASK_EXPOSURE
                        | xcb::EVENT_MASK_BUTTON_PRESS
                        | xcb::EVENT_MASK_BUTTON_RELEASE
                        | xcb::EVENT_MASK_BUTTON_1_MOTION,
                ),
            ],
        );

        // Display the window
        xcb::map_window(&conn, window_handle);

        // Flush the X connection queue so that all the commands go through
        conn.flush();

        // Save all the inputs for later
        self.draw_context = Some(draw_context);
        self.window_handle = Some(window_handle);
        self.window_dimensions = Some(dimensions);
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
    /// Destroy the window.
    fn drop(&mut self) {
        // TODO: Drop stuff
        // (removed unimplemented!() call for now so the plugin doesn't crash)
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
