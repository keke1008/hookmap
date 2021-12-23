use hookmap_core::{ButtonEvent, MouseCursorEvent, MouseWheelEvent, NativeEventOperation};

pub(crate) trait Hook<E> {
    fn native_event_operation() -> NativeEventOperation;
    fn run(&self, event: E);
}

pub(crate) trait HookStorage {
    type ButtonHook: Hook<ButtonEvent>;
    type MouseCursorHook: Hook<MouseCursorEvent>;
    type MouseWheelHook: Hook<MouseWheelEvent>;

    fn fetch_button_hook(&self, event: ButtonEvent) -> Vec<Self::ButtonHook>;
    fn fetch_mouse_cursor_hook(&self, event: MouseCursorEvent) -> Vec<Self::MouseCursorHook>;
    fn fetch_mouse_wheel_hook(&self, event: MouseWheelEvent) -> Vec<Self::MouseWheelHook>;
}
