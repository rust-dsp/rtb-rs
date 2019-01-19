use log::*;

use crate::event::Event;
use crate::window::WindowImpl;
use crate::event::EventHandler;

use std::borrow::Borrow;
use std::ffi::c_void;
use std::sync::{Arc, Mutex};
use std::thread;

mod x_handle;

pub struct PlatformWindow {
    x_handle: x_handle::XHandle,
    draw_context: u32,
    window_handle: u32,
    state: Arc<Mutex<Option<WindowState>>>,
}

pub struct WindowState {
    pub name: String,
    pub event_handler: Box<dyn EventHandler + Send>,
}

impl PlatformWindow {
    fn set_window_state(&mut self, state: WindowState) {
        let mut x = self.state.lock().unwrap();
        *x = Some(state);
    }
}

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

        // Start handling events
        let state = Arc::new(Mutex::new(None));
        let thread_conn = x_handle.conn();
        let thread_state = state.clone();
        thread::spawn(move || {
            handle_events(thread_conn, thread_state);
        });

        Self {
            x_handle,
            draw_context,
            window_handle,
            state,
        }
    }

    fn add_events_hook(&mut self, event_handler: Box<dyn EventHandler + Send>) {
        let mut x = self.state.lock().unwrap();
        *x = Some(WindowState{
            name: "derp".into(),
            event_handler,
        });
    }
}

impl Drop for PlatformWindow {
    /// Destroy the window.
    fn drop(&mut self) {
        // TODO: Drop stuff
        // (removed unimplemented!() call for now so the plugin doesn't crash)
        info!("WindowImpl<X11>::drop()");
    }
}

fn handle_events(conn: Arc<xcb::Connection>, state: Arc<Mutex<Option<WindowState>>>) {
    loop {
        let wait = conn.wait_for_event();
        if let Some(event) = wait {
            // If we don't have a state yet (and therefore no callback to call), don't do anything.
            if state.lock().unwrap().is_none() {
                continue;
            }

            // Otherwise, handle the event (by calling the callback)
            match event.response_type() {
                xcb::BUTTON_PRESS => {
                    info!("xcb::BUTTON_PRESS");
                    let mut mutex_lock = state.lock().unwrap();
                    let mut window_state = mutex_lock.as_mut().unwrap();

                    window_state.event_handler.handle(Event::LeftMouseDown);
                }
                _ => {
                    // Ignore unknown events
                }
            }
        }
    }
}
