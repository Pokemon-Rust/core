use crate::graphics::actor::Actor;
use crate::engine::engine::{SharedState};
use ggez::GameResult;
use std::cell::{RefCell};

pub mod player;

// define types
pub type Script = fn(&mut Actor, &RefCell<SharedState>) -> GameResult;