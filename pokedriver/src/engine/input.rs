use ggez::Context;
use std::cell::RefCell;
use crate::engine::engine::SharedState;
use ggez::input::keyboard;
use std::collections::HashSet;
use ggez::event::KeyCode;

pub struct Input {
    pressed_keys: HashSet<KeyCode>
}

impl Input {
    pub fn new() -> Input {
        Input {
            pressed_keys: HashSet::new()
        }
    }

    pub fn capture(&mut self, ctx: &mut Context, state: &RefCell<SharedState>) {
        // handle key-down events.
        let pressed_keys = keyboard::pressed_keys(ctx);

        let controller = &mut state.borrow_mut().controller;
        controller.set_pressed_keys(pressed_keys.clone());

        let mut unpressed_keys :HashSet<KeyCode> = HashSet::new();

        // handle key-up events.
        for keycode in &self.pressed_keys {
            if !pressed_keys.contains(keycode) {
                unpressed_keys.insert(*keycode);
            }
        }

        controller.set_unpressed_keys(unpressed_keys.clone());

        self.pressed_keys = pressed_keys.clone();
    }
}