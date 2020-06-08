use amethyst::{
    core::bundle::SystemBundle,
    ecs::prelude::{DispatcherBuilder, World},
    error::Error,
};

use crate::system::tile::static_tile::StaticTileSystem;

pub struct StaticTileBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for StaticTileBundle {
    fn build(self, _world: &mut World, dispatcher: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        let mut system = StaticTileSystem::new();

        dispatcher.add(system, "static_tile_system", &[]);
        Ok(())
    }
}