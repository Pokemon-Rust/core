use std::cell::RefCell;

use cgmath::Point2;
use ggez::{Context, GameResult, graphics};

use crate::engine::controller::{Controller, ControllerOwnership, KeyEvent};
use crate::engine::engine::SharedState;
use crate::graphics::Component;
use crate::graphics::components::ComponentIdentity;
use crate::graphics::overworld::ViewPort;

#[derive(ControllerOwnership)]
pub struct Dialog {
    text: Vec<String>,
    current_text: i16,
    dialog_type: DialogType,
}

impl Dialog {
    pub fn new(text: Vec<String>, dialog_type: DialogType) -> GameResult<Dialog> {
        let dialog = Dialog {
            text,
            dialog_type,
            current_text: 0,
        };
        Ok(dialog)
    }
}

impl Component for Dialog {
    fn update(&mut self, state: &RefCell<SharedState>) -> GameResult<()> {
        if self.own() {

        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, view_port: &ViewPort) -> GameResult<()> {}

    fn location(&self) -> Point2<f32> {
        let (width, height) = graphics::drawable_size(ctx);
        Point2 {
            x: width + 0.1,
            y: height + height,
        }
    }

    fn id(&self) -> ComponentIdentity {
        ComponentIdentity::Dialog(self.dialog_type)
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum DialogType {
    TalkDialog
}