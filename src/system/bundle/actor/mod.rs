mod player;


use amethyst::{
    core::bundle::SystemBundle,
    ecs::prelude::{DispatcherBuilder, World},
    error::Error,
};

use crate::system::bundle::actor::player::PlayerBundle;

pub struct ActorBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for ActorBundle {
    fn build(self, world: &mut World, dispatcher: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        let player_bundle = PlayerBundle;
        player_bundle.build(world, dispatcher)?;

        Ok(())
    }
}