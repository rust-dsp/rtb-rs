#![allow(dead_code)]
#![allow(non_snake_case)]

use objc::runtime::{Object, Sel};
use cocoa::appkit::NSEvent;
use cocoa::base::{ id, YES, BOOL };

/*
The bare-minimum steps required for an NSView to recieve events:
    - Override acceptsFirstResponder.
    - Implement one or more NSResponder methods to handle events of particular types.
    - If your view is to handle action messages passed to it via the responder chain, implement the appropriate action methods.

https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/EventOverview/EventHandlingBasics/EventHandlingBasics.html#//apple_ref/doc/uid/10000060i-CH5-SW11
*/

pub extern "C" fn do_nsevent(_this: &Object, _: Sel, event: id) {
    println!("NSEvent type: {:?}", unsafe { NSEvent::eventType(event) });
}

// @property(readonly) BOOL acceptsFirstResponder;
// A view that is first responder accepts key events and action messages before other objects in a window.
pub extern fn acceptsFirstResponder(_: &Object, _: Sel) -> BOOL {
    println!("acceptsFirstResponder");
    YES
}

// func acceptsFirstMouse(for event: NSEvent?) -> Bool
pub extern fn acceptsFirstMouse(_: &Object, _: Sel, _theEvent: id) -> BOOL {
    println!("acceptsFirstMouse");
    YES
}
