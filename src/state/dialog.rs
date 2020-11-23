use amethyst::{
    prelude::*,
    shred::FetchMut,
};

use crate::entity::dialog::talk_dialog::TalkDialog;
use crate::state::{Game, Trigger};


pub struct DialogState {}

impl DialogState {
    pub fn new() -> Self {
        DialogState {}
    }

    fn initialize_dialog(&mut self, world: &mut World) {
        TalkDialog::create(world);
    }

    fn fetch_game<'s>(&mut self, world: &'s mut World) -> FetchMut<'s, Game> {
        world.write_resource::<Game>()
    }
}

impl SimpleState for DialogState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        self.initialize_dialog(world);


        println!("DialogState: on_start()");
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let world = &mut data.world;

        let mut trigger = None;
        let mut dead_entities = Vec::new();

        {
            let mut game = self.fetch_game(world);
            trigger = game.get_trigger();
            dead_entities = game.dead_entities.clone();
            game.dead_entities.clear();
            game.clear_trigger();
        }

        for entity in &dead_entities {
            world.delete_entity(*entity);
        }

        if trigger.is_some() {
            let mut trans = Trans::None;

            match trigger.unwrap() {
                Trigger::DialogEnd => { trans = Trans::Pop; }
            };

            trans
        } else {
            Trans::None
        }
    }
}