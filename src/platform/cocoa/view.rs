#![allow(non_snake_case)]

use cocoa::appkit::*;
use cocoa::base::*;

use objc::runtime::{ Object, Sel, Class };
use objc::declare::{ ClassDecl };

use std::sync::{Once, ONCE_INIT};

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
                acceptsFirstResponder as extern "C" fn(&Object, Sel) -> BOOL);
            extern "C" fn acceptsFirstResponder(_this: &Object, _sel: Sel) -> BOOL { YES }

            // Overridden by subclasses to return true if the view should be sent a mouseDown(with:)
            // message for an initial mouse-down event
            decl.add_method(sel!(acceptsFirstMouse:),
                acceptsFirstMouse as extern fn(this: &Object, _: Sel, _: id) -> BOOL);
            extern "C" fn acceptsFirstMouse(_this: &Object, _sel: Sel, _: id) -> BOOL { println!("acceptsFirstMouse"); YES }

            decl.add_method(sel!(scrollWheel:), processNSEvent as extern "C" fn(&Object, Sel, id));
            decl.add_method(sel!(mouseDown:),   processNSEvent as extern "C" fn(&Object, Sel, id));
            decl.add_method(sel!(mouseUp:),     processNSEvent as extern "C" fn(&Object, Sel, id));
            decl.add_method(sel!(mouseMoved:),  processNSEvent as extern "C" fn(&Object, Sel, id));
            decl.add_method(sel!(keyDown:),     processNSEvent as extern "C" fn(&Object, Sel, id));
            decl.add_method(sel!(keyUp:),       processNSEvent as extern "C" fn(&Object, Sel, id));
            extern "C" fn processNSEvent(_this: &Object, _: Sel, event: id) {
                println!("NSEvent type: {:?}", unsafe { NSEvent::eventType(event) });
            }

            decl.add_method(sel!(applicationDidFinishLaunching:),
                did_finish_launching as extern "C" fn(&Object, Sel, id));
            extern "C" fn did_finish_launching(_this: &Object, _sel: Sel, _notification: id) {
                println!("applicationDidFinishLaunching");
            }

            DELEGATE_CLASS = decl.register();
        }
    });
    unsafe { DELEGATE_CLASS }
}
