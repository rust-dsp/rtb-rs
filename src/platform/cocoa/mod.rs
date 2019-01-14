extern crate cocoa;

use crate::platform::cocoa::view::*;
use crate::window::{Size, WindowImpl};

use core::ffi::c_void;

use cocoa::appkit::{ NSWindow, NSView };
use cocoa::base::{ id, nil, YES };
use cocoa::foundation::{ NSString, NSSize };

mod view;
mod util;

pub struct PlatformWindow {
    pub window: id,
    pub view: id,
}

pub struct WindowState {
    pub name: String,
    pub event_cb: Box<Fn(crate::event::Event)>,
}

impl PlatformWindow {
    // fn get_window_state<'a>(&self) -> &'a WindowState {
    //     unsafe {
    //         let delegate: id = msg_send![self.window, delegate];
    //         let ivar: *const c_void = *(&*delegate).get_ivar("windowState");
    //         &*(ivar as *const WindowState)
    //     }
    // }

    fn set_window_state(&self, state: WindowState) {
        let mut state = Box::new(state);
        let state_ptr: *mut WindowState = &mut *state;

        unsafe {
            let delegate: id = msg_send![self.window, delegate];
            (&mut *delegate).set_ivar("windowState", state_ptr as *mut ::std::os::raw::c_void);
            std::mem::forget(state);
        }
    }
}

impl WindowImpl for PlatformWindow {

    fn attach(parent: *mut c_void) -> PlatformWindow {
        unsafe {
            // todo: structure this setup code properly.

            // window
            let window:id = util::get_window_id(parent as id);
            window.setAcceptsMouseMovedEvents_(YES);
            window.makeKeyAndOrderFront_(nil);
            window.setOpaque_(YES);
            window.center();

            // content view
            let content_view = window.contentView();

            // main view
            let view: id = msg_send![view_class(), new];
            window.setDelegate_(view);
            window.makeFirstResponder_(view);
            NSView::initWithFrame_(view, NSView::frame(content_view as id));
            content_view.addSubview_(view);

            let red = msg_send![class!(NSColor), redColor];
            view.setBackgroundColor_(red);

            util::resize_view_to_window(view);

            PlatformWindow {
                window: window,
                view: view,
            }
        }
    }

    fn add_events_hook(&mut self, events: Box<Fn(crate::event::Event)>) {
        let state = WindowState{ event_cb: events, name: "Rob".to_string() };
        self.set_window_state(state);
    }

    fn resize(&mut self, size: Size) {
        let nssize = NSSize::new(size.width as f64, size.height as f64);

        unsafe {
            let mut frame = NSWindow::frame(self.window);
            frame.size = nssize;
            self.window.setFrame_display_(frame, YES);

            util::resize_view_to_window(self.view);
        }
    }

    fn set_title(&mut self, title: &str) {
        unsafe {
            let nstitle = NSString::alloc(nil).init_str(title);
            self.window.setTitle_(nstitle);
        }
    }
}

impl Drop for PlatformWindow {
    fn drop(&mut self) { }
}
