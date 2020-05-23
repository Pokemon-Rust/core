use std::cell::RefCell;

use cgmath::Point2;
use ggez::GameResult;
use ggez::graphics::Font;

use crate::engine::controller::{Controller, ControllerOwnership, KeyEvent};
use crate::engine::engine::SharedState;
use crate::graphics::components::ComponentIdentity;
use crate::graphics::dialog::{DialogType, DialogAttrs};
use crate::scripts::dialog::DialogBehaviour;

#[derive(ControllerOwnership)]
pub struct TalkDialog {
}

impl DialogBehaviour for TalkDialog {
    fn run(&mut self, attrs: &mut DialogAttrs, state: &RefCell<SharedState>) -> GameResult<()> {
        if self.own(state) && attrs.visible {
            if attrs.text_index + 1 == attrs.text.len() {
                attrs.visible = true;
            } else {
                // self.current_text += 1;
            }
        }

        self.disown(state);
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