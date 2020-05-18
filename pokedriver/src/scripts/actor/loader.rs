use crate::scripts::actor::ActorBehaviour;
use crate::scripts::actor;

pub enum ActorBehaviourType {
    Player
}

pub fn load(key: &ActorBehaviourType) -> Box<dyn ActorBehaviour> {
    match key {
        ActorBehaviourType::Player => Box::new(actor::player::PlayerBehaviour::new().speed(4.0))
    }
}