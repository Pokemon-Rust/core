use crate::graphics::actor::Actor;
use crate::engine::engine::{SharedState};
use ggez::{GameResult, Context};
use std::cell::{RefCell};

pub mod player;
pub mod loader;

// define types
pub type Script = fn(&mut Context, &mut Actor, &RefCell<SharedState>) -> GameResult;