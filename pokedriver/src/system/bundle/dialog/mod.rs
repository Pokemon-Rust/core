pub mod talk;


use amethyst::{
    core::bundle::SystemBundle,
    ecs::prelude::{DispatcherBuilder, World},
    error::Error,
};
use crate::system::bundle::tile::static_tile::StaticTileBundle;
use crate::system::bundle::dialog::talk::TalkDialogBundle;

pub struct DialogBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for DialogBundle {
    fn build(self, world: &mut World, dispatcher: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        let talk_dialog_bundle = TalkDialogBundle;
        talk_dialog_bundle.build(world, dispatcher)?;

        Ok(())
    }
}
