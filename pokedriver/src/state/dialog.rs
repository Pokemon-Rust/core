use amethyst::{
    prelude::*,
    window::ScreenDimensions,
    core::{transform::Transform},
    renderer::camera::{Camera, Projection},
};

use crate::entity::actor::player::Player;
use crate::utils::debug;
use crate::entity::tile::{tile::Tile};
use crate::entity::dialog::talk_dialog::TalkDialog;


pub struct DialogState {

}

impl DialogState {
    pub fn new() -> Self {
        DialogState {}
    }

    fn initialize_dialog(&mut self, world: &mut World) {
        TalkDialog::create(world);
    }

}

impl SimpleState for DialogState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        self.initialize_dialog(world);
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::None
    }
}