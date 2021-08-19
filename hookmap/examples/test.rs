use hookmap::*;

fn main() {
    let hook = Hook::new();

    // Bind Ctrl + W,A,S,D as cursor key
    let mod_ctrl = hook
        .cond(&Cond::pressed(&Button::Ctrl))
        .cond(&Cond::released(&Button::Shift));
    mod_ctrl.bind(&Button::W).like(&Button::UpArrow);
    mod_ctrl.bind(&Button::A).like(&Button::LeftArrow);
    mod_ctrl.bind(&Button::S).like(&Button::DownArrow);
    mod_ctrl.bind(&Button::D).like(&Button::RightArrow);

    // Mouse wheel rotation with Ctrl + Shift + W,A
    let ctrl_shift = ButtonSet::new(&[Button::Ctrl, Button::Shift]);
    let mod_c_s = hook.cond(&Cond::pressed(&ctrl_shift.all())).block();
    mod_c_s.bind(&Button::W).on_press(|_| Mouse::rotate(1));
    mod_c_s.bind(&Button::S).on_press(|_| Mouse::rotate(-1));

    hook.handle_input();
}
