use objc::runtime::{ Object, Sel, Class };
use objc::declare::{ ClassDecl };
use cocoa::base::{ id, BOOL };

use std::sync::{Once, ONCE_INIT};

use super::callbacks::*;

pub fn view_class() -> *const Class {
    static mut DELEGATE_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = ONCE_INIT;

    INIT.call_once(|| {
        let superclass = class!(NSView);
        let mut decl = ClassDecl::new("MainView", superclass).unwrap();

        // decl.add_ivar::<*mut c_void>("ViewController");
        // decl.add_ivar::<*mut c_void>("EventHandler");

        unsafe {

            decl.add_method(sel!(acceptsFirstResponder),
                acceptsFirstResponder as extern fn(this: &Object, _: Sel) -> BOOL);

            // Overridden by subclasses to return true if the view should be sent a mouseDown(with:)
            // message for an initial mouse-down event
            decl.add_method(sel!(acceptsFirstMouse:),
                acceptsFirstMouse as extern fn(this: &Object, _: Sel, _: id) -> BOOL);

            decl.add_method(sel!(scrollWheel:), do_nsevent as extern "C" fn(&Object, Sel, id));
            decl.add_method(sel!(mouseDown:),   do_nsevent as extern "C" fn(&Object, Sel, id));
            decl.add_method(sel!(mouseUp:),     do_nsevent as extern "C" fn(&Object, Sel, id));
            decl.add_method(sel!(mouseMoved:),  do_nsevent as extern "C" fn(&Object, Sel, id));
            decl.add_method(sel!(keyDown:),     do_nsevent as extern "C" fn(&Object, Sel, id));
            decl.add_method(sel!(keyUp:),       do_nsevent as extern "C" fn(&Object, Sel, id));

            DELEGATE_CLASS = decl.register();
        }
    });
    unsafe { DELEGATE_CLASS }
}
