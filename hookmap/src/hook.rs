use hookmap_core::event::{ButtonEvent, CursorEvent, NativeEventOperation, WheelEvent};

pub(crate) trait Hook<E> {
    fn native_event_operation(&self) -> NativeEventOperation;
    fn run(&self, event: E);
}

pub(crate) trait HookStorage {
    type ButtonHook: Hook<ButtonEvent>;
    type MouseCursorHook: Hook<CursorEvent>;
    type MouseWheelHook: Hook<WheelEvent>;

    fn fetch_button_hook(&self, event: ButtonEvent) -> Vec<Self::ButtonHook>;
    fn fetch_mouse_cursor_hook(&self, event: CursorEvent) -> Vec<Self::MouseCursorHook>;
    fn fetch_mouse_wheel_hook(&self, event: WheelEvent) -> Vec<Self::MouseWheelHook>;
}
