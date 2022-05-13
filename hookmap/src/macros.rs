//! Items used in macros.

#[doc(hidden)]
pub mod button_arg;

#[doc(hidden)]
pub mod sequence;

/// Expands button names.
///
/// If the argument is enclosed in square brackets, it will be expanded without any action.
///
/// # Example
/// ```no_run
/// use hookmap::{button_name, device::Button};
/// assert_eq!(Button::Key0, button_name!(0));
/// assert_eq!(Button::A, button_name!(A));
///
/// let button_a = Button::A;
/// assert_eq!(Button::A, button_name!([button_a]));
/// ```
///
// Using `#[rustfmt_skip]` instead, the following error is generated.
// error: macro-expanded `macro_export` macros from the current crate cannot be referred to by absolute paths
#[allow(clippy::deprecated_cfg_attr)]
#[cfg_attr(rustfmt, rustfmt_skip)]
#[macro_export]
#[doc(hidden)]
macro_rules! button_name {
    ([$button:expr]) => ($button);
    ($button:ident)  => ($crate::device::Button::$button);
    (0)              => ($crate::device::Button::Key0);
    (1)              => ($crate::device::Button::Key1);
    (2)              => ($crate::device::Button::Key2);
    (3)              => ($crate::device::Button::Key3);
    (4)              => ($crate::device::Button::Key4);
    (5)              => ($crate::device::Button::Key5);
    (6)              => ($crate::device::Button::Key6);
    (7)              => ($crate::device::Button::Key7);
    (8)              => ($crate::device::Button::Key8);
    (9)              => ($crate::device::Button::Key9);
}
