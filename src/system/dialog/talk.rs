use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, Write, System, SystemData, WriteStorage, Entities},
    input::{InputHandler, StringBindings},
    ui::{UiText},
};
use crate::entity::dialog::talk_dialog::TalkDialog;
use crate::utils::resolve;
use crate::state::{Game, Trigger};

#[derive(SystemDesc)]
pub struct TalkDialogSystem {
    counter: usize,
    speed: f32,
    capframes: f32,
    hold: bool,
}

impl TalkDialogSystem {
    pub fn new() -> Self {
        let mut system = TalkDialogSystem {
            counter: 0,
            speed: 15.0,
            capframes: 0.0,
            hold: false,
        };

        system.capframes = resolve::get_fps() as f32 / system.speed;
        system
    }

    fn set_text(&self, ui_text: &mut UiText, dialog: &TalkDialog) {
        ui_text.text = dialog.text[dialog.index.clone()][0..dialog.char_index.clone() + 1].to_string();
    }

    fn destroy_mesh(&self, entities: &Entities, dialog: &mut TalkDialog) {
        if dialog.mesh.is_some() {
            entities.delete(dialog.mesh.unwrap());
            dialog.mesh = None;
        }
    }
}

impl<'s> System<'s> for TalkDialogSystem {
    type SystemData = (
        WriteStorage<'s, TalkDialog>,
        WriteStorage<'s, UiText>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        Write<'s, Game>,
        Entities<'s>
    );

    fn run(&mut self, (mut dialogs, mut ui_texts, mut transforms, input, mut game, mut entities): Self::SystemData) {
        let should_continue = input.action_is_down("continue").unwrap_or(false) ||
                                    input.action_is_down("cancel").unwrap_or(false);


        if should_continue {
            self.capframes = resolve::get_fps() as f32 / (2.0 * self.speed);
        } else {
            self.capframes = resolve::get_fps() as f32 / self.speed;
        }

        for (dialog, ui_text, entity) in (&mut dialogs, &mut ui_texts, &*entities).join() {
            let is_line_end = dialog.char_index == dialog.text[dialog.index.clone()].len() - 1;

            if should_continue && is_line_end {
                self.hold = true;
            }

            if self.counter >= self.capframes as usize {
                if dialog.char_index < dialog.text[dialog.index.clone()].len() - 1 {
                    dialog.char_index += 1;
                }
                self.counter = 0;
            }

            self.counter += 1;

            if is_line_end && self.hold && !should_continue {
                self.hold = false;

                dialog.char_index = 0;
                if dialog.index < dialog.text.len() - 1 {
                    dialog.index += 1;
                } else {
                    if dialog.mesh.is_some() {
                        game.kill_entity(dialog.mesh.unwrap());
                    }
                    game.kill_entity(entity);
                    game.set_trigger(Trigger::DialogEnd);
                }
            }

            self.set_text(ui_text, dialog);
            if dialog.mesh.is_some() {
                for (e, trans) in (&*entities, &mut transforms).join() {
                    if e == dialog.mesh.unwrap() {
                        let cam_trans = game.camera_trans.clone();
                        trans.set_translation_xyz(cam_trans[0] + 320.0, cam_trans[1] - 640.0 + 80.0, 3.0);
                    }
                }
            }
        }
    }
}