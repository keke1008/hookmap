//! Definition of utility hotkeys.

use crate::macros::button_arg::ButtonArg;
use crate::macros::sequence::Sequence;
use crate::prelude::*;

fn bind_alt_tab_inner(
    hotkey: &mut Hotkey,
    context: &Context,
    alt: impl Into<ButtonArg> + Clone,
    tab: impl Into<ButtonArg> + Clone,
    tab_seq: Sequence,
) {
    hotkey
        .register(context.clone())
        .on_release(&alt, move |_| seq!(LAlt up).send());

    hotkey
        .register(Context::new().merge(context).modifiers(alt))
        .disable(&tab)
        .on_press(tab, move |_| {
            if Button::LAlt.is_released() {
                seq!(LAlt down).send();
            }
            tab_seq.send();
        });
}

/// Alt-Tab hotkey.
///
/// # Arguments
///
/// * `alt` - A button that act like Alt key.
/// * `tab` - A button that act like tab key.
///
/// # Example
///
/// ```
/// use hookmap::prelude::*;
///
/// let mut hotkey = Hotkey::new();
/// utils::alt_tab(&mut hotkey, &Context::new(), Button::A, Button::T);
/// ```
///
pub fn alt_tab(
    hotkey: &mut Hotkey,
    context: &Context,
    alt: impl Clone + Into<ButtonArg>,
    tab: impl Into<ButtonArg> + Clone,
) {
    bind_alt_tab_inner(hotkey, context, alt, tab, seq!(Tab));
}

/// Shift-Alt-Tab hotkey.
///
/// # Arguments
///
/// * `alt` - A button that act like Alt key.
/// * `tab` - A button that act like tab key.
///
/// # Example
///
/// ```
/// use hookmap::prelude::*;
///
/// let mut hotkey = Hotkey::new();
/// utils::shift_alt_tab(&mut hotkey, &Context::new(), Button::A, Button::T);
/// ```
///
pub fn shift_alt_tab(
    hotkey: &mut Hotkey,
    context: &Context,
    alt: impl Into<ButtonArg> + Clone,
    tab: impl Into<ButtonArg> + Clone,
) {
    bind_alt_tab_inner(hotkey, context, alt, tab, seq!(with(LShift), Tab));
}
