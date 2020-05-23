use std::cell::RefCell;

use cgmath::Point2;
use ggez::GameResult;

use crate::engine::controller::{Controller, ControllerOwnership, KeyEvent};
use crate::engine::engine::SharedState;
use crate::graphics::components::ComponentIdentity;
use crate::graphics::dialog::{DialogType, DialogAttrs};
use crate::scripts::dialog::DialogBehaviour;
use ggez::event::KeyCode;
use crate::graphics::fsync::FSync;
use crate::utils::resolver;


#[derive(ControllerOwnership)]
pub struct TalkDialog {
    key_event: KeyEvent,
    fsync: FSync,
    triggered: bool
}

enum TalkDialogAction {
    Continue,
    Cancel,
}

impl TalkDialog {
    pub fn new() -> TalkDialog {
        TalkDialog {
            fsync: FSync::new().set_frames(resolver::get_fps()),
            key_event: KeyEvent::new(),
            triggered: true
        }
    }

    fn evaluate(&mut self, state: &RefCell<SharedState>, attr: &DialogAttrs) {
        let controller = &mut state.borrow_mut().controller;
        if self.key_event.handled && self.is_valid_dialog_keydown(controller) {
            self.key_event.handled = false;
            self.set_dialog_key(controller)
        }

        if !self.key_event.handled {
            if self.is_valid_dialog_keyup(controller) {
                self.triggered = true;
                self.handle_keyup(controller);
            }
        }

        self.handle_keydown(controller);
    }

    fn handle_keydown(&self, controller: &mut Controller) {
        controller.handle_keydown(KeyCode::Z);
        controller.handle_keydown(KeyCode::X);
    }

    fn handle_keyup(&self, controller: &mut Controller) {
        controller.handle_keyup(KeyCode::Z);
        controller.handle_keyup(KeyCode::X);
    }

    fn is_valid_dialog_keydown(&self, controller: &Controller) -> bool {
        controller.is_keydown(KeyCode::Z) ||
            controller.is_keydown(KeyCode::X)
    }

    fn is_valid_dialog_keyup(&self, controller: &Controller) -> bool {
        controller.is_keyup(KeyCode::Z) ||
            controller.is_keyup(KeyCode::X)
    }

    fn set_dialog_key(&mut self, controller: &Controller) {
        let mut keycode = self.key_event.key;

        if !controller.is_keydown(keycode) {
            if controller.is_keydown(KeyCode::Z) {
                keycode = KeyCode::Z;
            } else if controller.is_keydown(KeyCode::X) {
                keycode = KeyCode::X;
            }
        }

        self.key_event.key = keycode;
    }

    fn map_key_action(&self, keycode: KeyCode) -> Option<TalkDialogAction> {
        match keycode {
            KeyCode::Z => Some(TalkDialogAction::Continue),
            KeyCode::X => Some(TalkDialogAction::Cancel),
            _ => None
        }
    }

    #[inline]
    fn try_handle(&mut self) {
        if self.fsync.get_event_frame() == resolver::get_fps() - 1 {
            self.key_event.handled = true;
        }
    }

    fn handle_action(&self, action: Option<TalkDialogAction>, attrs: &mut DialogAttrs, state: &RefCell<SharedState>) {
        if let Some(dialog_action) = action {
            match dialog_action {
                TalkDialogAction::Continue => {
                    if attrs.text_index + 1 == attrs.text.len() {
                        attrs.visible = false;
                        attrs.text_index = 0;
                        self.disown(state);
                    } else {
                        attrs.text_index += 1;
                    }
                }
                TalkDialogAction::Cancel => {
                    attrs.visible = false;
                    attrs.text_index = 0;
                    self.disown(state);
                }
            }
        }
    }
}

impl DialogBehaviour for TalkDialog {
    fn run(&mut self, attrs: &mut DialogAttrs, state: &RefCell<SharedState>) -> GameResult<()> {
        if attrs.visible {
            if self.own(state) {
                self.evaluate(state, attrs);

                if self.triggered {
                    let pressed_key = self.key_event.key;

                    let action = self.map_key_action(pressed_key);
                    self.handle_action(action, attrs, state);
                    self.triggered = false;

                    self.try_handle();
                    self.fsync.update();
                }
            }
        }

        Ok(())
    }

    fn transform_location(&mut self, state: &RefCell<SharedState>, location: &mut Point2<f32>) {
        let cstate = state.borrow_mut();
        let width = cstate.view_port.width;
        let height = cstate.view_port.height;


        *location = Point2 {
            x: cstate.view_port.origin.x + width / 256.0,
            y: cstate.view_port.origin.y + height * 0.75,
        }
    }

    fn id(&self) -> ComponentIdentity {
        ComponentIdentity::Dialog(DialogType::TalkDialog)
    }
}