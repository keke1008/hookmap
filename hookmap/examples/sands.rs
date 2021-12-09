use hookmap::{
    button::ButtonSet,
    interceptor::{Filter, Interceptor},
    *,
};
use std::collections::HashSet;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

fn emulate_sands<T, U>(hook: &T, space: U, ignore: HashSet<Button>)
where
    T: SelectHandleTarget + SetNativeEventOpereation,
    U: ButtonInput + Into<ButtonSet> + Clone + Send + Sync + 'static,
{
    let is_alone = Arc::new(AtomicBool::new(true));

    hotkey!(hook => {
        block_event {
            on_press [&space] => {
                let is_alone = Arc::clone(&is_alone);
                move |_| {
                    is_alone.store(true, Ordering::SeqCst);
                    seq!(LShift down);
                }
            };

            on_release [&space] => {
                let is_alone = Arc::clone(&is_alone);
                move |_| {
                    if is_alone.load(Ordering::SeqCst) { space.click(); }
                    seq!(LShift up);
                }
            };
        }
    });

    let filter = Filter::new().action(ButtonAction::Press);
    Interceptor::unblock(filter).then_iter(move |iter| {
        iter.filter(|e| !ignore.contains(&e.target))
            .for_each(|_| is_alone.store(false, Ordering::SeqCst));
    });
}

fn main() {
    let hotkey = Hotkey::new();
    let ignore = [
        Button::Space,
        Button::LShift,
        Button::RShift,
        Button::LCtrl,
        Button::RCtrl,
        Button::LAlt,
        Button::RAlt,
        Button::LMeta,
        Button::RMeta,
    ]
    .iter()
    .copied()
    .collect();

    emulate_sands(&hotkey, Button::Space, ignore);

    hotkey.handle_input();
}
