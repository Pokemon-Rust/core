use ggez::event::KeyCode;
use std::collections::HashSet;
use crate::graphics::components::ComponentIdentity;
use std::cell::RefCell;
use crate::engine::engine::SharedState;

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

pub struct Controller {
    pressed_keys: HashSet<KeyCode>,
    unpressed_keys: HashSet<KeyCode>,
    locked: bool,
    component_id: ComponentIdentity,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            pressed_keys: HashSet::new(),
            unpressed_keys: HashSet::new(),
            locked: false,
            component_id: ComponentIdentity::World,
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

    pub fn handle_keydown(&mut self, keycode: KeyCode) {
        self.pressed_keys.remove(&keycode);
    }

    pub fn handle_keyup(&mut self, keycode: KeyCode) {
        self.unpressed_keys.remove(&keycode);
    }

    // Tries to lock the Controller, if lock fails returns false, otherwise true.
    pub fn try_lock(&mut self, component_id: ComponentIdentity) -> bool {
        if !self.locked || self.component_id == component_id {
            self.component_id = component_id;
            self.locked = true;
            true
        } else {
            false
        }
    }

    // Unlocks the controller, if and only if the component owns the controller.
    pub fn unlock(&mut self, component_id: ComponentIdentity) {
        if self.component_id == component_id {
            self.locked = false;
        }
    }
}

pub trait ControllerOwnership {
    fn own(&self, state: &RefCell<SharedState>) -> bool;

    fn disown(&self, state: &RefCell<SharedState>);
}