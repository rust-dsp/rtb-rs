extern crate cocoa;

use crate::platform::cocoa::view::*;
use crate::mouse::*;
use crate::window::{Size, WindowImpl};

use core::ffi::c_void;

use cocoa::appkit::{ NSWindow, NSView };
use cocoa::base::{ id, nil, YES };
use cocoa::foundation::{ NSString, NSSize };

mod view;
mod callbacks;
mod util;

pub struct PlatformWindow {
    pub window: id,
    pub view: id,
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

            PlatformWindow {
                window: window,
                view: view,
            }
        }
    }

    fn resize(&mut self, size: Size) {
        let nssize = NSSize::new(size.width as f64, size.height as f64);

        unsafe {
            let mut frame = NSWindow::frame(self.window);
            frame.size = nssize;
            self.window.setFrame_display_(frame, YES);
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

impl MouseHandler for PlatformWindow {}
