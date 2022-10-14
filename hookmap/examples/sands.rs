use std::sync::Arc;

use hookmap::prelude::*;
use Button::*;

fn sands(hotkey: &Hotkey) {
    let (space_pressed_layer, space_pressed) = hotkey.create_inheritance_layer(false);
    let space_pressed_layer = Arc::new(space_pressed_layer);
    let space_pressed_layer_ = Arc::clone(&space_pressed_layer);

    hotkey
        .disable(Space)
        .on_press(Space, move |_| {
            space_pressed_layer.enable();
            Shift.press();
        })
        .on_release(Space, |_| Shift.release());

    let ignore = not![Space, LShift, RShift, LCtrl, RCtrl, LAlt, RAlt, LSuper, RSuper];

    space_pressed
        .on_press(ignore, move |_| space_pressed_layer_.disable())
        .on_release_raw(Space, |_| Space.click());
}

fn main() {
    let hotkey = Hotkey::new();
    sands(&hotkey);
    hotkey.install();
}
