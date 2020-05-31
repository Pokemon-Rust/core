use crate::system::actor::player::PlayerSystem;


use amethyst::{
    core::bundle::SystemBundle,
    ecs::prelude::{DispatcherBuilder, World},
    error::Error,
};
use crate::system::actor::behaviour::walk::Walk;

pub struct PlayerBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for PlayerBundle {
    fn build(self, _world: &mut World, dispatcher: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        let mut system = PlayerSystem::new();
        system.add_behaviour(Walk::new());

        dispatcher.add(system, "player_system", &["input_system"]);
        Ok(())
    }
}