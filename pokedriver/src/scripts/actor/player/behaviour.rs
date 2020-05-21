// Actor-script for Player.

use crate::graphics::actor::ActorAttributes;
use crate::engine::engine::SharedState;
use ggez::GameResult;
use std::cell::RefCell;
use crate::scripts::actor::ActorBehaviour;
use cgmath::Point2;
use crate::scripts::actor::player::walk::WalkBehaviour;
use crate::graphics::components::ComponentIdentity;
use crate::scripts::actor::player::PlayerBehaviourType;


pub struct PlayerBehaviour {
    walk: Box<dyn ActorBehaviour>
}


impl PlayerBehaviour {
    pub fn new() -> PlayerBehaviour {
        PlayerBehaviour {
            walk: Box::new(WalkBehaviour::new().speed(4.0))
        }
    }
}

impl ActorBehaviour for PlayerBehaviour {
    fn run(&mut self, state: &RefCell<SharedState>, attr: &mut ActorAttributes) -> GameResult<()> {
        self.walk.run(state, attr)?;
        Ok(())
    }

    fn transform_location(&mut self, state: &RefCell<SharedState>, location: &mut Point2<f32>) {
        self.walk.transform_location(state, location);
    }

    fn id(&self) -> ComponentIdentity {
        ComponentIdentity::Player(PlayerBehaviourType::None)
    }
}