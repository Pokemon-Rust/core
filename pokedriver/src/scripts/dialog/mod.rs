use std::cell::RefCell;
use crate::engine::engine::SharedState;
use ggez::GameResult;
use cgmath::Point2;
use crate::graphics::components::ComponentIdentity;
use crate::graphics::dialog::DialogAttrs;

pub mod talk_dialog;


pub trait DialogBehaviour {
    fn run(&mut self, attrs: &mut DialogAttrs, state: &RefCell<SharedState>) -> GameResult<()>;
    fn transform_location(&mut self, state: &RefCell<SharedState>, location: &mut Point2<f32>);
    fn id(&self) -> ComponentIdentity;
}