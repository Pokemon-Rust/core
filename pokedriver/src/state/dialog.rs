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
use crate::state::{Game, Trigger};


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


        println!("DialogState: on_start()");
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        data.world.maintain();
        let mut game = data.world.write_resource::<Game>();
        let trigger = game.get_trigger();

        if trigger.is_some() {
            let trans = match trigger.unwrap() {
                Trigger::DialogEnd => Trans::Pop
            };

            game.clear_trigger();
            trans
        } else {
            Trans::None
        }
    }
}