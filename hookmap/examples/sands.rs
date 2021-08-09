use hookmap::*;

fn emulate_sands(hook: &impl SelectHandleTarget, space: Button) {
    space.block_input();
    hook.bind(space).like(Button::Shift);
    hook.modifier(space);
    hook.bind(space).on_release_alone(move |_| space.click());
}

fn main() {
    let hook = Hook::new();

    emulate_sands(&hook, Button::Space);

    hook.handle_input();
}
