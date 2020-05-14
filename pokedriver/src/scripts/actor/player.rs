// Actor-script for Player.

use crate::graphics::actor::Actor;
use crate::engine::engine::{SharedState};
use ggez::GameResult;
use std::cell::RefCell;

//todo: Implement navigation

pub fn run(actor: &mut Actor, state: &RefCell<SharedState>) -> GameResult<()> {
    println!("pokedriver: executed scripts::actor::player.rs");

    Ok(())
}