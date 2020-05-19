use ggez::event::KeyCode;
use std::collections::HashSet;

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
pub struct KeyEvent {
    pub key: KeyCode,
    pub handled: bool,
}

impl KeyEvent {
    pub fn new() -> KeyEvent {
        KeyEvent {
            key: KeyCode::Escape,
            handled: true,
        }
    }
}

// The controller is an embodiment of all types of controls i.e. keyboard, mouse, joystick etc.
// The game-scripts can use the controller for relevant control-events and handle them if they are
// valid to their corresponding context.

// Here, the controller maintains a KeyEvent which remains alive till one of the following events occur:
// 1. The script handles the key-event.
// The assumption is that the frame-rate is faster than the player's response time.

pub struct Controller {
    pressed_keys: HashSet<KeyCode>,
    unpressed_keys: HashSet<KeyCode>,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            pressed_keys: HashSet::new(),
            unpressed_keys: HashSet::new(),
        }
    }

    pub fn is_keydown(&self, keycode: KeyCode) -> bool {
        self.pressed_keys.contains(&keycode)
    }

    pub fn is_keyup(&self, keycode: KeyCode) -> bool {
        self.unpressed_keys.contains(&keycode)
    }

    pub fn set_pressed_keys(&mut self, set: HashSet<KeyCode>) {
        self.pressed_keys = set;
    }

    pub fn set_unpressed_keys(&mut self, set: HashSet<KeyCode>) {
        self.unpressed_keys = set;
    }

    pub fn get_pressed_keys(&self) -> HashSet<KeyCode> {
        self.pressed_keys.clone()
    }

    pub fn get_unpressed_keys(&self) -> HashSet<KeyCode> {
        self.unpressed_keys.clone()
    }

}