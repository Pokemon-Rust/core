use ggez::event::KeyCode;

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct KeyEvent {
    pub keycode: KeyCode,
    pub handled: bool,
}

impl KeyEvent {
    pub fn new() -> KeyEvent {
        KeyEvent {
            keycode: KeyCode::Escape,
            handled: true,
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
    key_down_events: Vec<KeyEvent>,
    key_up_events: Vec<KeyEvent>,
    buffer_size: usize
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            key_down_events: Vec::new(),
            key_up_events: Vec::new(),
            buffer_size: 5
        }
    }

    pub fn set_key_down_event(&mut self, keycode: KeyCode) {
        if self.key_down_events.len() == self.buffer_size {
            self.handle_key_down_event()
        }
        self.key_down_events.push(KeyEvent {
            keycode,
            handled: false,
        });
    }

    pub fn get_key_down_event(&self) -> KeyEvent {
        if self.key_down_events.len() > 0 {
            self.key_down_events[0].clone()
        } else {
            KeyEvent::new()
        }
    }

    pub fn handle_key_down_event(&mut self) {
        if self.key_down_events.len() > 0 {
            self.key_down_events.remove(0);
        }
    }

    pub fn set_key_up_event(&mut self, keycode: KeyCode) {
        if self.key_up_events.len() == self.buffer_size {
            self.handle_key_up_event()
        }

        self.key_up_events.push(KeyEvent {
            keycode,
            handled: false,
        });
    }

    pub fn get_key_up_event(&self) -> KeyEvent {
        if self.key_up_events.len() > 0 {
            self.key_up_events[0].clone()
        } else {
            KeyEvent::new()
        }
    }

    pub fn handle_key_up_event(&mut self) {
        if self.key_up_events.len() > 0 {
            self.key_up_events.remove(0);
        }
    }

    pub fn clear_key_down_events(&mut self, index: usize) {
        // remove all key_down events except the first one
        let mut count = self.key_down_events.len() as i32 - index as i32;
        while count > 0 {
            self.key_down_events.remove(index);
            count -= 1;
        }
    }

    pub fn clear_key_up_events(&mut self, index: usize) {
        // remove all key_up events except the first one
        let mut count = self.key_up_events.len() as i32  - index as i32;
        while count > 0 {
            self.key_up_events.remove(index);
            count -= 1;
        }
    }

    pub fn peek_key_down_event(&self) -> KeyEvent {
        let length = self.key_down_events.len();
        if length > 0 {
            self.key_down_events[length- 1]
        } else {
            KeyEvent::new()
        }
    }

    pub fn peek_key_up_event(&self) -> KeyEvent {
        let length = self.key_up_events.len();
        if length > 0 {
            self.key_up_events[length- 1]
        } else {
            KeyEvent::new()
        }
    }
}