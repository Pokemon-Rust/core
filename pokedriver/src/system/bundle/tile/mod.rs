pub mod static_tile;

use amethyst::{
    core::bundle::SystemBundle,
    ecs::prelude::{DispatcherBuilder, World},
    error::Error,
};
use crate::system::bundle::tile::static_tile::StaticTileBundle;

pub struct TileBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for TileBundle {
    fn build(self, world: &mut World, dispatcher: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        let static_tile_bundle = StaticTileBundle;
        static_tile_bundle.build(world, dispatcher)?;

        Ok(())
    }
}