#[derive(Clone, Debug)]
pub enum Event {
    LeftMouseDown,
    LeftMouseUp,
    RightMouseDown,
    RightMouseUp,
    OtherMouseDown,
    OtherMouseUp,
    KeyDown,
    KeyUp,
}

pub trait EventHandler {
    fn handle(&self, event: Event);
}
