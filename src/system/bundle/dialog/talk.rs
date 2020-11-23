use amethyst::{
    core::bundle::SystemBundle,
    ecs::prelude::{DispatcherBuilder, World},
    error::Error,
};

use crate::system::dialog::talk::TalkDialogSystem;

pub struct TalkDialogBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for TalkDialogBundle {
    fn build(self, _world: &mut World, dispatcher: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        let system = TalkDialogSystem::new();

        dispatcher.add(system, "talk_dialog_system", &["input_system"]);
        Ok(())
    }
}