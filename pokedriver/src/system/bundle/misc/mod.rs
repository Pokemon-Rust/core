pub mod camera_translation;

use amethyst::{
    core::bundle::SystemBundle,
    ecs::prelude::{DispatcherBuilder, World},
    error::Error,
};

use crate::system::bundle::misc::camera_translation::CameraTranslationBundle;

pub struct MiscBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for MiscBundle {
    fn build(self, world: &mut World, dispatcher: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        let camera_trans_bundle = CameraTranslationBundle;
        camera_trans_bundle.build(world, dispatcher)?;

        Ok(())
    }
}
