//! Definition of utility hotkeys.

use std::sync::Arc;

use crate::hotkey::args::ButtonArgs;
use crate::macros::sequence::Sequence;
use crate::prelude::*;

fn alt_tab_inner(hotkey: &Hotkey, tab: ButtonArgs, tab_seq: Sequence) {
    let (alt_tab_layer, alt_tab_active) = hotkey.create_inheritance_layer(false);
    let alt_tab_layer = Arc::new(alt_tab_layer);
    let alt_tab_layer_ = Arc::clone(&alt_tab_layer);

    hotkey.on_press(tab, move |_| {
        alt_tab_layer.enable();
        Button::Alt.press();
        tab_seq.send();
    });
    alt_tab_active.on_layer_inactivated(move |_| {
        alt_tab_layer_.disable();
        Button::Alt.release();
    });
}

pub trait HotkeyExt {
    fn alt_tab(&self, tab: impl Into<ButtonArgs>) -> &Self;
    fn shift_alt_tab(&self, tab: impl Into<ButtonArgs>) -> &Self;
}

impl HotkeyExt for Hotkey<'_> {
    fn alt_tab(&self, tab: impl Into<ButtonArgs>) -> &Self {
        alt_tab_inner(self, tab.into(), seq![Tab]);
        self
    }

    fn shift_alt_tab(&self, tab: impl Into<ButtonArgs>) -> &Self {
        alt_tab_inner(self, tab.into(), seq![with(LShift), Tab]);
        self
    }
}
