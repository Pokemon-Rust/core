mod actor;
mod tile;
mod dialog;

use amethyst::{
    core::bundle::SystemBundle,
    ecs::prelude::{DispatcherBuilder, World},
    error::Error,
};


use crate::system::bundle::actor::ActorBundle;
use crate::system::bundle::tile::TileBundle;
use crate::system::bundle::dialog::DialogBundle;

pub struct GameBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GameBundle {
    fn build(self, world: &mut World,
             dispatcher: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {

        let actor_bundle = ActorBundle;
        actor_bundle.build(world, dispatcher)?;

        let tile_bundle = TileBundle;
        tile_bundle.build(world, dispatcher)?;

        let dialog_bundle = DialogBundle;
        dialog_bundle.build(world, dispatcher);


        Ok(())
    }
}