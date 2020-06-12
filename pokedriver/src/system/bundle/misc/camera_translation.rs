use amethyst::{
    core::bundle::SystemBundle,
    ecs::prelude::{DispatcherBuilder, World},
    error::Error,
};

use crate::system::misc::camera_translation::CameraTranslationSystem;

pub struct CameraTranslationBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for CameraTranslationBundle  {
    fn build(self, _world: &mut World, dispatcher: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        let system = CameraTranslationSystem::new();

        dispatcher.add(system, "camera_translation_system", &[]);
        Ok(())
    }
}