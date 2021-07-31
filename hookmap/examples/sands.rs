use hookmap::*;

fn emulate_sands(hook: &Hook, space: Key) {
    hook.bind_key(space).as_key(Key::Shift);
    hook.modifier_key(space, EventBlock::Block);
    hook.bind_key(space)
        .on_release_alone(move |_| space.click());
}

fn main() {
    let hook = Hook::new();

    emulate_sands(&hook, Key::Space);

    hook.handle_input();
}
