use hookmap::macros::button_arg::ButtonArg;
use hookmap::prelude::*;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

fn emulate_sands(hotkey: &mut Hotkey, context: &Context, space: Button, ignored: ButtonArg) {
    let is_other_key_pressed = Arc::new(AtomicBool::default());

    let mut registrar = hotkey.register(
        Context::new()
            .merge(context)
            .native_event_operation(NativeEventOperation::Block),
    );

    let is_other_key_pressed_ = Arc::clone(&is_other_key_pressed);
    registrar.on_press(space, move |_| {
        is_other_key_pressed_.store(false, Ordering::SeqCst);
        seq!(LShift down).send();
    });

    let is_other_key_pressed_ = Arc::clone(&is_other_key_pressed);
    registrar.on_release(space, move |_| {
        seq!(LShift up).send();
        if !is_other_key_pressed_.load(Ordering::SeqCst) {
            seq!([space]).send();
        }
    });

    let target = buttons!(![ignored]);
    let filter = Filter::new().action(ButtonAction::Press).target(target);

    std::thread::spawn(move || {
        Interceptor::dispatch(filter)
            .iter()
            .for_each(|_| is_other_key_pressed.store(true, Ordering::SeqCst))
    });
}

fn main() {
    let mut hotkey = Hotkey::new();
    let ignored = buttons!(Space, LShift, RShift, LCtrl, RCtrl, LAlt, RAlt, LShift, RShift);

    emulate_sands(&mut hotkey, &Context::default(), Button::Space, ignored);

    hotkey.install();
}
