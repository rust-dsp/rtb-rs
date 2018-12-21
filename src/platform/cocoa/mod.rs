extern crate cocoa;

use crate::mouse::*;
use crate::window::{Size, WindowImpl};

use core::ffi::c_void;
use cocoa::appkit::NSWindow;
use cocoa::base::{ id, nil };
use cocoa::foundation::{ NSString, NSSize };

pub struct PlatformWindow {
    pub id: id
}

impl WindowImpl for PlatformWindow {
    fn attach(parent: *mut c_void) -> PlatformWindow {
        PlatformWindow {
            id: parent as id
        }
    }

    fn resize(&mut self, size: Size) {
        unsafe { self.id.setContentSize_(
            NSSize::new(size.width as f64, size.height as f64));
        }
    }

    fn set_title(&mut self, title: &str) {
        unsafe {
            let nstitle = NSString::alloc(nil).init_str(title);
            self.id.setTitle_(nstitle);
        }
    }
}

impl Drop for PlatformWindow {
    fn drop(&mut self) { }
}

impl MouseHandler for PlatformWindow {}
