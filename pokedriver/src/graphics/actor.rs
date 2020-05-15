use std::cell::RefCell;
use std::collections::HashMap;

use cgmath::mint::Point2;
use cgmath::mint::Vector2;
use ggez::{Context, GameResult, graphics};
use ggez::graphics::DrawParam;

use crate::engine::engine::SharedState;
use crate::scripts::actor;
use crate::utils::resolver;
use crate::engine::timer;
use std::sync::{Arc, Mutex};
use std::borrow::Borrow;

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum ActorDirection {
    North,
    South,
    East,
    West,
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum ActorAction {
    Stand,
    Walk1,
    Walk2,
    // todo: add more types for surf, running, bicycling etc.
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct ActorAttributes {
    pub direction: ActorDirection,
    pub action: ActorAction,
}


// The Actor struct encapsulates all the sprites corresponding to the actor.
// The script uses the SharedState to operate properly,
// i.e. Bound actor to TileMaps, etc. Finally, the script updates the actor attributes using
// &mut Actor. The actor::update() function executes the script at first, then the actor::draw() fn
// renders the sprite corresponding to the ActorAttribute specified by the script,
// ny changes to the storyline can be made
// from the script using a mutable reference to the SharedState.

pub trait ActorBehaviour {
    fn run(&mut self, shared_state: Arc<Mutex<SharedState>>) -> GameResult<()>;
    // fn from(ctx: &mut Context, actor: &String, attribute_batch: &Vec<ActorAttributes>) -> GameResult<Box<dyn ActorBehaviour>>;
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()>;
}



