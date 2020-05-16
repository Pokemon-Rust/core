use crate::graphics::actor::ActorAttributes;
use crate::engine::engine::{SharedState};
use ggez::GameResult;
use std::cell::RefCell;

pub mod player;
pub mod loader;

pub trait ActorBehaviour {
    fn run(&mut self, attr: &mut ActorAttributes, state: &RefCell<SharedState>) -> GameResult<()>;
}