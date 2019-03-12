extern crate log;
#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

pub mod event;
pub mod platform;
pub mod window;

pub use self::event::*;
pub use self::window::*;
