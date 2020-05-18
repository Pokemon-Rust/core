use crate::graphics::actor::ActorAttributes;
use crate::engine::engine::{SharedState};
use ggez::GameResult;
use cgmath::Point2;
use std::cell::RefCell;

pub mod player;
pub mod loader;

pub trait ActorBehaviour {
    fn run(&mut self, state: &RefCell<SharedState>, attr: &mut ActorAttributes) -> GameResult<()>;
    fn transform_location(&mut self, state: &RefCell<SharedState>, location: &mut Point2<f32>);
}