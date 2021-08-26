use hookmap::*;

fn main() {
    let hook = Hook::new();

    // Binds the H,J,K,L keys as in vim.
    hotkey!(hook => {
        bind H => LeftArrow;
        bind J => DownArrow;
        bind K => UpArrow;
        bind L => RightArrow;
    });

    let mod_ctrl = hook.cond(Cond::pressed(&CTRL)).block();
    hotkey!(mod_ctrl => {
        // Disables the Mouse cursor movement while the Shift key is held down.
        disable MouseMove;

        // Send Ctrl+A when the Shift and the Space key are pressed.
        on_press Space => |_| send!(LCtrl down, A, LCtrl up);
    });

    let a_or_b = button_set!(A, B).any();
    hotkey!(mod_ctrl => {
                             // Called when the A key or the B key is pressed.
        on_press [&a_or_b] => |event| println!("{:?} key is pressed.", event.target);

        // Sends an uppercase A or B when the A or B key is released.
        on_release [a_or_b] => |event| send!(LShift down, [event.target], LShift up);
    });

    hook.handle_input();
}
