use cocoa::base::*;
use cocoa::foundation::*;

pub fn id_is_instance_of(id: id, classname: &'static str) -> bool {
    let is_instance: BOOL = unsafe {
        msg_send![id, isKindOfClass: objc::runtime::Class::get(classname).expect("class to exist")]
    };
    is_instance == YES
}

/// get an nswindow id given either an nswindow or nsview
pub unsafe fn get_window_id(id:id) -> id {
    match id_is_instance_of(id, "NSWindow") {
        true => id,
        false => msg_send![id, window],
    }
}

pub unsafe fn resize_view_to_window(view: id) {
    // TODO: use nsscreen method instead
    let window:id = msg_send![view, window];
    let content_view:id = msg_send![window, contentView];
    let content_frame:NSRect = msg_send![content_view, frame];

    msg_send![view, setFrame:content_frame];
}
