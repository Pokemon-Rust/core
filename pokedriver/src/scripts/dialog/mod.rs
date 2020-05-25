use std::cell::RefCell;
use crate::engine::engine::SharedState;
use ggez::GameResult;
use crate::graphics::components::ComponentIdentity;
use crate::graphics::dialog::DialogAttrs;

pub mod talk_dialog;


pub trait DialogBehaviour {
    fn run(&mut self, attrs: &mut DialogAttrs, state: &RefCell<SharedState>) -> GameResult<()>;
    fn id(&self) -> ComponentIdentity;
}