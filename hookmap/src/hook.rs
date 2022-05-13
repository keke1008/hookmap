use hookmap_core::button::Button;
use hookmap_core::event::{ButtonEvent, CursorEvent, NativeEventOperation, WheelEvent};

pub(crate) trait Hook<E> {
    fn native_event_operation(&self) -> NativeEventOperation;
    fn run(&self, event: E);
}

pub(crate) trait ButtonState {
    fn is_pressed(&self, button: Button) -> bool;
    fn is_released(&self, button: Button) -> bool;
}

pub(crate) trait HookStorage {
    type ButtonHook: Hook<ButtonEvent>;
    type MouseCursorHook: Hook<CursorEvent>;
    type MouseWheelHook: Hook<WheelEvent>;

    fn fetch_button_hook<S: ButtonState>(
        &self,
        event: ButtonEvent,
        state: &S,
    ) -> Vec<Self::ButtonHook>;

    fn fetch_mouse_cursor_hook<S: ButtonState>(
        &self,
        event: CursorEvent,
        state: &S,
    ) -> Vec<Self::MouseCursorHook>;

    fn fetch_mouse_wheel_hook<S: ButtonState>(
        &self,
        event: WheelEvent,
        state: &S,
    ) -> Vec<Self::MouseWheelHook>;
}
