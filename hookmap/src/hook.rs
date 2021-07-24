use crate::{
    event::Button,
    handler::Handler,
    modifier::Modifier,
    register::{KeyboardRegister, MouseRegister},
};
use hookmap_core::{
    EventBlock, Key, KeyboardAction, KeyboardEvent, MouseAction, MouseEvent, MouseInput,
    INPUT_HANDLER,
};
use std::sync::{Arc, Mutex};

/// A struct that handles generated input events.
#[derive(Debug, Default)]
pub struct Hook {
    handler: Arc<Mutex<Handler>>,
    modifier: Modifier,
}

impl Hook {
    /// Creates a new instance of `Hook`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a [`KeyboardRegister`] for assigning a function to the key passed as an arguments.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::{Hook, Key};
    /// let hook = Hook::new();
    /// hook.bind_key(Key::A).on_press(|_| println!("The A key is pressed."));
    /// ```
    ///
    pub fn bind_key(&self, key: Key) -> KeyboardRegister {
        KeyboardRegister::new(key, self.modifier.clone(), Arc::clone(&self.handler))
    }

    /// Returns a [`MouseRegister`] for assigning a function to the mouse as an argument.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::{Hook, MouseInput};
    /// let hook = Hook::new();
    /// hook.bind_mouse(MouseInput::Wheel).on_rotate(|_| println!("The mouse wheel rotated."));
    /// ```
    ///
    pub fn bind_mouse(&self, mouse: MouseInput) -> MouseRegister {
        MouseRegister::new(mouse, self.modifier.clone(), Arc::clone(&self.handler))
    }

    /// Returns a new instance of `Hook`.
    /// The hooks assigned through this instance will be active only when the given key is pressed.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::{Hook, Key};
    /// let hook = Hook::new();
    /// let modifier_space = hook.modifier_key(Key::Space);
    /// modifier_space
    ///     .bind_key(Key::A)
    ///     .on_press(|_| println!("The A key is pressed while the Space key is pressed."));
    /// ```
    ///
    pub fn modifier_key(&self, key: Key) -> Self {
        Self {
            modifier: self.modifier.added_key(key),
            handler: Arc::clone(&self.handler),
        }
    }

    /// Returns a new instance of `Hook`.
    /// The hooks assigned through this instance will be active only when the given mouse button is pressed.
    ///
    /// # Example
    ///
    /// ```
    /// use hookmap::{Hook, Key, MouseInput};
    /// let hook = Hook::new();
    /// let modifier_left = hook.modifier_mouse_button(MouseInput::LButton);
    /// modifier_left
    ///     .bind_key(Key::A)
    ///     .on_press(|_| println!("The A key is pressed while the left mouse button is pressed."));
    /// ```
    ///
    pub fn modifier_mouse_button(&self, mouse_button: MouseInput) -> Self {
        Self {
            modifier: self.modifier.added_mouse_button(mouse_button),
            handler: Arc::clone(&self.handler),
        }
    }

    /// Handles input events and blocks the current thread.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use hookmap::{Hook, Key};
    /// let hook = Hook::new();
    /// hook.bind_key(Key::A).on_press(|_| println!("The A key is pressed"));
    /// hook.handle_input(); // Blocking the current thread.
    /// ```
    ///
    pub fn handle_input(self) {
        INPUT_HANDLER
            .keyboard
            .lock()
            .unwrap()
            .register_handler(self.create_keyboard_handler());
        INPUT_HANDLER
            .mouse
            .lock()
            .unwrap()
            .register_handler(self.create_mouse_handler());
        INPUT_HANDLER.handle_input();
    }

    fn create_keyboard_handler(&self) -> impl FnMut(KeyboardEvent) -> EventBlock {
        let handler = Arc::clone(&self.handler);
        move |event: KeyboardEvent| {
            let handler = &mut handler.lock().unwrap().keyboard;
            let key = event.target;
            let event_block = match event.action {
                KeyboardAction::Press => {
                    let mut event_block = handler
                        .on_press_or_release
                        .call_available(key, Button::Press);
                    event_block.append(&mut handler.on_press.call_available(key, ()));
                    event_block
                }
                KeyboardAction::Release => {
                    let mut event_block = handler
                        .on_press_or_release
                        .call_available(key, Button::Release);
                    event_block.append(&mut handler.on_release.call_available(key, ()));
                    event_block
                }
            };

            if event_block.into_iter().any(|e| e == EventBlock::Block) {
                EventBlock::Block
            } else {
                EventBlock::Unblock
            }
        }
    }

    fn create_mouse_handler(&self) -> impl FnMut(MouseEvent) -> EventBlock {
        let handler = Arc::clone(&self.handler);
        move |event: MouseEvent| {
            let handler = &mut handler.lock().unwrap().mouse;
            let mouse = event.target;
            let event_block = match event.action {
                MouseAction::Press => {
                    let mut event_block = handler
                        .on_press_or_release
                        .call_available(mouse, Button::Press);
                    event_block.append(&mut handler.on_press.call_available(mouse, ()));
                    event_block
                }
                MouseAction::Release => {
                    let mut event_block = handler
                        .on_press_or_release
                        .call_available(mouse, Button::Release);
                    event_block.append(&mut handler.on_release.call_available(mouse, ()));
                    event_block
                }
                MouseAction::Wheel(value) => handler.wheel.call_available(value),
                MouseAction::Move(value) => handler.cursor.call_available(value),
            };

            if event_block.into_iter().any(|e| e == EventBlock::Block) {
                EventBlock::Block
            } else {
                EventBlock::Unblock
            }
        }
    }
}
