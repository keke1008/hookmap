#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use self::windows::mouse;
#[cfg(target_os = "windows")]
pub(crate) use self::windows::{install, uninstall};
