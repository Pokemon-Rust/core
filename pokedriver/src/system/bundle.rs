use crate::system::actor::player::PlayerSystem;
use amethyst::{
    core::bundle::SystemBundle,
    ecs::prelude::{DispatcherBuilder, World},
    error::Error,
};

pub struct GameBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GameBundle {
    fn build(self, _world: &mut World,
             builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {

        builder.add(PlayerSystem, "player_system", &[]);
        Ok(())
    }
}