use crate::common::handler::{HandleInput, InputHandler};
use std::{mem::MaybeUninit, ptr};
use winapi::um::winuser;

impl HandleInput for InputHandler {
    fn handle_input() {
        unsafe {
            winuser::GetMessageW(MaybeUninit::zeroed().assume_init(), ptr::null_mut(), 0, 0);
        }
    }
}
