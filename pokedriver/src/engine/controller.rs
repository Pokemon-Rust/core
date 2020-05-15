use ggez::event::KeyCode;

#[derive(Clone)]
pub struct KeyEvent {
    pub keycode: KeyCode,
    pub handled: bool
}

impl KeyEvent {
    pub fn new() -> KeyEvent {
        KeyEvent {
            keycode: KeyCode::Escape,
            handled: true
        }
    }
}

// The controller is an embodiment of all types of controls i.e. keyboard, mouse, joystick etc.
// The game-scripts can use the controller for relevant control-events and handle them if they are
// valid to their corresponding context.

// Here, the controller maintains a KeyEvent which remains alive till one of the following events occur:
// 1. The script handles the key-event.
// 2. The player performs a new key-event which overwrites the existing key-event.
// The assumption is that the frame-rate is faster than the player's response time.

pub struct Controller {
    key_down_event: KeyEvent,
    key_up_event: KeyEvent
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            key_down_event: KeyEvent::new(),
            key_up_event: KeyEvent::new()
        }
    }

    pub fn set_key_down_event(&mut self, keycode: KeyCode) {
        self.key_down_event.keycode = keycode;
        self.key_down_event.handled = false;
    }

    pub fn get_key_down_event(&self) -> KeyEvent {
        self.key_down_event.clone()
    }

    pub fn handle_key_down_event(&mut self) {
        self.key_down_event.handled = true;
    }

    pub fn set_key_up_event(&mut self, keycode: KeyCode) {
        self.key_up_event.keycode = keycode;
        self.key_up_event.handled = false;
    }

    pub fn get_key_up_event(&self) -> KeyEvent {
        self.key_up_event.clone()
    }

    pub fn handle_key_up_event(&mut self) {
        self.key_up_event.handled = true;
    }
}