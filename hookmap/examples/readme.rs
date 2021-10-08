use hookmap::*;

fn main() {
    let hotkey = Hotkey::new();

    hotkey!(hotkey => {

        // Remap H,J,K,L keys as in vim.
        remap H => LeftArrow;
        remap J => DownArrow;
        remap K => UpArrow;
        remap L => RightArrow;


        // if left ctrl is pressed and right shift is not pressed.
        modifier(LCtrl, !RShift) {

            // Disables the Mouse cursor movement.
            disable MouseMove;

            // Disable the event so that it does not reach other processes.
            block_event {

                // Send Ctrl+A when the Shift and the Space key are pressed.
                on_press Space => |_| send!(with(LCtrl), A);

                // Sends an uppercase A or B when the A or B key is pressed.
                on_release [any!(A, B)] => |event| {
                    send!(with(LShift, [event.target]));
                };
            }
        }
    });

    hotkey.handle_input();
}
