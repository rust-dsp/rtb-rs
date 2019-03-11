use crate::event::Event;
use crate::window::{Size, WindowImpl};
use std::ffi::c_void;
use std::ptr::{null, null_mut};

use winapi::shared::basetsd::*;
use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::um::libloaderapi::*;
use winapi::um::winuser::*;

// We need to wrap an event handler in a structure because we cannot obtain a
// C-style pointer from the 'Box<Fn(Event)>' value. The pointer is needed to
// pass the event handler to the window procedure.
struct EventHandler {
    handler: Box<Fn(Event)>
}

impl EventHandler {
    fn new() -> Self {
        EventHandler {
            handler: Box::new(|_: Event| ())
        }
    }
}

pub struct PlatformWindow {
    handle: HWND,
    event_handler: Box<EventHandler>,
}

impl PlatformWindow {
    // The "rtb_rs_window" string in utf16.
    const CLASS_NAME: [u16; 14] = [
        0x0072, 0x0074, 0x0062, 0x005f, 0x0072, 0x0073, 0x005f, 0x0077,
        0x0069, 0x006e, 0x0064, 0x006f, 0x0077, 0x0000];

    fn new(parent: HWND) -> PlatformWindow {
        PlatformWindow::register_window_class();

        let handle: HWND = unsafe {
            CreateWindowExW(
                0,
                PlatformWindow::CLASS_NAME.as_ptr(),
                null(),
                WS_CHILD | WS_VISIBLE,
                0,
                0,
                0,
                0,
                parent,
                null_mut(),
                GetModuleHandleW(null()),
                null_mut())
        };

        PlatformWindow {
            handle: handle,
            event_handler: Box::new(EventHandler::new())
        }
    }

    fn register_window_class() {
        let class = WNDCLASSW {
            style: 0,
            lpfnWndProc: Some(PlatformWindow::window_procedure),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: unsafe {
                GetModuleHandleW(null())
            },
            hIcon: null_mut(),
            hCursor: unsafe {
                LoadCursorW(null_mut(), IDC_ARROW)
            },
            hbrBackground: (COLOR_WINDOW + 1) as HBRUSH,
            lpszMenuName: null(),
            lpszClassName: PlatformWindow::CLASS_NAME.as_ptr()
        };

        unsafe {
            RegisterClassW(&class);
        }
    }

    extern "system" fn window_procedure(
        handle: HWND, message: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT
    {
        let event_handler: *const EventHandler = unsafe {
            GetWindowLongPtrW(handle, GWLP_USERDATA) as *const EventHandler
        };

        if event_handler != null() {
            let handler: &Box<Fn(Event)> = unsafe {
                &(*event_handler).handler
            };

            match message {
                WM_LBUTTONDOWN => {
                    handler(Event::LeftMouseDown);
                },
                WM_LBUTTONUP => {
                    handler(Event::LeftMouseUp);
                },
                WM_RBUTTONDOWN => {
                    handler(Event::RightMouseDown);
                },
                WM_RBUTTONUP => {
                    handler(Event::RightMouseUp);
                },
                WM_MBUTTONDOWN | WM_XBUTTONDOWN => {
                    handler(Event::OtherMouseDown);
                },
                WM_MBUTTONUP | WM_XBUTTONUP => {
                    handler(Event::OtherMouseUp);
                },
                WM_GETDLGCODE => {
                    return DLGC_WANTALLKEYS;
                },
                _ => ()
            }
        }

        unsafe {
            DefWindowProcW(handle, message, wparam, lparam)
        }
    }
}

impl WindowImpl for PlatformWindow {
    fn attach(parent: *mut c_void) -> Self {
        PlatformWindow::new(parent as HWND)
    }

    fn add_events_hook(&mut self, event_handler: Box<Fn(Event)>) {
        self.event_handler = Box::new(EventHandler {handler: event_handler});

        unsafe {
            SetWindowLongPtrW(
                self.handle,
                GWLP_USERDATA,
                &*self.event_handler as *const _ as LONG_PTR);
        }
    }

    fn resize(&mut self, size: Size) {
        unsafe {
            SetWindowPos(
                self.handle,
                null_mut(),
                0,
                0,
                size.width,
                size.height,
                SWP_NOACTIVATE | SWP_NOZORDER | SWP_NOMOVE);
        }
    }
}
