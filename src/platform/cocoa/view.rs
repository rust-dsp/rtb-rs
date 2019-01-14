#![allow(non_snake_case)]

use core::ffi::c_void;
use cocoa::appkit::*;
use cocoa::base::*;
use cocoa::appkit;

use objc::runtime::{ Object, Sel, Class };
use objc::declare::{ ClassDecl };

use std::sync::{Once, ONCE_INIT};

use crate::event::Event;
unsafe fn ns_event_to_event(ns_event: cocoa::base::id) -> Option<Event> {
    if ns_event == cocoa::base::nil {
        return None;
    }

    // thankyou winit team for finding this cocoa bug. see winit src.
    if ns_event.eventType() as u64 == 21 { return None; }

    let event_type = ns_event.eventType();
    match event_type {
        appkit::NSKeyDown => Some(Event::KeyDown),
        appkit::NSKeyUp => Some(Event::KeyUp),
        appkit::NSLeftMouseDown => Some(Event::LeftMouseDown),
        appkit::NSLeftMouseUp => Some(Event::LeftMouseUp),
        appkit::NSRightMouseDown => Some(Event::RightMouseDown),
        appkit::NSRightMouseUp => Some(Event::RightMouseUp),
        appkit::NSOtherMouseDown => Some(Event::OtherMouseDown),
        appkit::NSOtherMouseUp => Some(Event::OtherMouseUp),
        _ => None
    }
}

pub fn view_class() -> *const Class {
    static mut DELEGATE_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = ONCE_INIT;

    INIT.call_once(|| {
        let superclass = class!(NSView);
        let mut decl = ClassDecl::new("MainView", superclass).unwrap();

        decl.add_ivar::<*mut c_void>("windowState");

        unsafe {

            decl.add_method(sel!(acceptsFirstResponder),
                acceptsFirstResponder as extern "C" fn(&Object, Sel) -> BOOL);
            extern "C" fn acceptsFirstResponder(_this: &Object, _sel: Sel) -> BOOL { YES }

            // Overridden by subclasses to return true if the view should be sent a mouseDown(with:)
            // message for an initial mouse-down event
            decl.add_method(sel!(acceptsFirstMouse:),
                acceptsFirstMouse as extern fn(this: &Object, _: Sel, nsevent_id: id) -> BOOL);
            extern "C" fn acceptsFirstMouse(_this: &Object, _sel: Sel, _: id) -> BOOL { println!("acceptsFirstMouse"); YES }

            decl.add_method(sel!(scrollWheel:), processNSEvent as extern "C" fn(&Object, Sel, id));
            decl.add_method(sel!(mouseDown:),   processNSEvent as extern "C" fn(&Object, Sel, id));
            decl.add_method(sel!(mouseUp:),     processNSEvent as extern "C" fn(&Object, Sel, id));
            decl.add_method(sel!(mouseMoved:),  processNSEvent as extern "C" fn(&Object, Sel, id));
            decl.add_method(sel!(keyDown:),     processNSEvent as extern "C" fn(&Object, Sel, id));
            decl.add_method(sel!(keyUp:),       processNSEvent as extern "C" fn(&Object, Sel, id));
            extern "C" fn processNSEvent(this: &Object, _: Sel, nsevent_id: id) {
                unsafe {
                    use super::*;
                    let window: id = msg_send![this, window];
                    let delegate: id = msg_send![window, delegate];
                    let ivar: *const c_void = *(&*delegate).get_ivar("windowState");
                    let state = &*(ivar as *const WindowState);

                    if let Some(event) = ns_event_to_event(nsevent_id) {
                        (state.event_cb)(event);
                    }
                }
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
