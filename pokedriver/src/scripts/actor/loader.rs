use crate::scripts::actor;
use crate::graphics::actor::{ActorBehaviour, ActorAttributes, ActorDirection, ActorAction};
use ggez::Context;

pub enum ScriptKey {
    Player
}

pub fn load(key: ScriptKey, ctx: &mut Context) -> Box<dyn ActorBehaviour> {
    match key {
        ScriptKey::Player => {
            let attr_batch = vec![
                ActorAttributes {
                    direction: ActorDirection::South,
                    action: ActorAction::Stand,
                }, ActorAttributes {
                    direction: ActorDirection::North,
                    action: ActorAction::Stand,
                }, ActorAttributes {
                    direction: ActorDirection::East,
                    action: ActorAction::Stand,
                }, ActorAttributes {
                    direction: ActorDirection::West,
                    action: ActorAction::Stand,
                }, ActorAttributes {
                    direction: ActorDirection::South,
                    action: ActorAction::Walk1,
                }, ActorAttributes {
                    direction: ActorDirection::North,
                    action: ActorAction::Walk1,
                }, ActorAttributes {
                    direction: ActorDirection::East,
                    action: ActorAction::Walk1,
                }, ActorAttributes {
                    direction: ActorDirection::West,
                    action: ActorAction::Walk1,
                },
                ActorAttributes {
                    direction: ActorDirection::South,
                    action: ActorAction::Walk2,
                }, ActorAttributes {
                    direction: ActorDirection::North,
                    action: ActorAction::Walk2,
                }, ActorAttributes {
                    direction: ActorDirection::East,
                    action: ActorAction::Walk2,
                }, ActorAttributes {
                    direction: ActorDirection::West,
                    action: ActorAction::Walk2,
                },
            ];
            Box::new(actor::player::PlayerActor::from(ctx, &"brendan".to_string(), attr_batch.as_ref()).unwrap())
        }
    }
}