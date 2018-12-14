/// Conditionally compile in platform base libs.
/// Only one if these should be true.
/// Each submodule should export the same types so that the upper layers can treat them all as the same thing
/// 

#[cfg(target_os="macos")]
mod cocoa;
#[cfg(target_os="macos")]
pub use self::cocoa::*;

//Should handle anything that does x11
//linux, unix, bsd, ...
#[cfg(all(unix, not(target_os="macos")))]
mod x11;
#[cfg(all(unix, not(target_os="macos")))]
pub use self::x11::*;

#[cfg(windows)]
mod win;
#[cfg(windows)]
pub use self::win::*;


#[cfg(all(not(unix), not(windows), not(target_os="macos")))]
mod stub;
#[cfg(all(not(unix), not(windows), not(target_os="macos")))]
pub use self::stub::*;










