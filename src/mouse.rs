pub struct Mouse {}

pub enum MouseCursor {}

pub enum MouseButton {}

pub enum MouseButtonState {}

pub trait MouseHandler {
    fn mouse_press(&mut self, button: MouseButton, x: isize, y: isize);
    fn mouse_release(&mut self, button: MouseButton, x: isize, y: isize);
    fn mouse_motion(&mut self, x: isize, y: isize);
    fn mouse_wheel(&mut self, x: isize, y: isize, delta: f32);
    fn mouse_enter_window(&mut self, x: isize, y: isize);
    fn mouse_leave_window(&mut self, x: isize, y: isize);

    fn mouse_double_click_interval(&self) -> i64;
    fn set_cursor(&mut self, mouse: &mut Mouse, cursor: MouseCursor);
    fn mouse_pointer_wrap(&self, x: isize, y: isize);
    fn copy_to_clipboard(&self, buffer: &[u8]);
    fn paste_from_clipboard(&self, buffer: &mut [u8]) -> isize;
}
