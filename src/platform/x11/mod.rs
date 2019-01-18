use log::*;

use crate::event::Event;
use crate::window::WindowImpl;
use std::borrow::Borrow;
use std::ffi::c_void;

mod x_handle;

pub struct PlatformWindow {
    x_handle: x_handle::XHandle,
    draw_context: u32,
    window_handle: u32,
}

impl PlatformWindow {}

impl WindowImpl for PlatformWindow {
    /// Create a window that is attached to the parent window.
    /// For X11, `parent` should be interpreted as a u32 window ID instead of a pointer to a handle.
    fn attach(parent: *mut c_void) -> Self {
        info!("WindowImpl<X11>::attach()");

        // Create an X11 handle
        let x_handle = x_handle::XHandle::new();

        // TODO: If there's no parent to attach to, just create a "root" window instead of attaching.
        /*
        if parent.is_null() {
        }
        */
        let parent_id = parent as u32;

        // Create a draw context
        let conn = x_handle.conn();
        let setup = conn.get_setup();
        let screen = setup.roots().nth(x_handle.screen_num() as usize).unwrap();
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
            0,    // TODO: use Size or WindowDimensions or whatever
            0,    // TODO: use Size or WindowDimensions or whatever
            1000, // TODO: use Size or WindowDimensions or whatever
            1000, // TODO: use Size or WindowDimensions or whatever
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

        Self {
            x_handle,
            draw_context,
            window_handle,
        }
    }

    fn add_events_hook(&mut self, _events: Box<Fn(Event)>) {}
}

impl Drop for PlatformWindow {
    /// Destroy the window.
    fn drop(&mut self) {
        // TODO: Drop stuff
        // (removed unimplemented!() call for now so the plugin doesn't crash)
        info!("WindowImpl<X11>::drop()");
    }
}
