use cocoa::base::*;

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
