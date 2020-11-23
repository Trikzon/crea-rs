use glfw::{Action, Key, Modifiers, MouseButton};

/// Provides an interface to mouse and keyboard input.
pub struct Input {
    cursor_x: f64,
    cursor_y: f64,
    mouse_buttons: [bool; 8],
    just_pressed_mb: Vec<MouseButton>,
    keys: [bool; 350],
    just_pressed_keys: Vec<Key>,
}

impl Input {
    pub(crate) fn new() -> Self {
        Input {
            cursor_x: 0.0, cursor_y: 0.0,
            mouse_buttons: [false; 8],
            keys: [false; 350],
            just_pressed_mb: vec![],
            just_pressed_keys: vec![],
        }
    }

    pub(crate) fn cursor_pos_event(&mut self, x: f64, y: f64) {
        self.cursor_x = x;
        self.cursor_y = y;
    }

    pub(crate) fn mouse_button_event(
        &mut self, button: MouseButton, action: Action, _modifiers: Modifiers
    ) {
        if !self.mouse_buttons[button as usize] && action == Action::Press {
            self.just_pressed_mb.push(button);
        }
        self.mouse_buttons[button as usize] = action == Action::Press;
    }

    pub(crate) fn key_event(&mut self, key: Key, action: Action, _modifiers: Modifiers) {
        if !self.keys[key as usize] && action == Action::Press {
            self.just_pressed_keys.push(key);
        }
        self.keys[key as usize] = action == Action::Press;
    }

    pub(crate) fn end_frame(&mut self) {
        self.just_pressed_mb.clear();
        self.just_pressed_keys.clear();
    }

    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.mouse_buttons[button as usize]
    }

    pub fn is_mouse_button_just_pressed(&self, button: MouseButton) -> bool {
        self.just_pressed_mb.contains(&button)
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.keys[key as usize]
    }

    pub fn is_key_just_pressed(&self, key: Key) -> bool {
        self.just_pressed_keys.contains(&key)
    }
}
