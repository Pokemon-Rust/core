use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::{SpriteRender, camera::Camera},
    core::math::Vector3,
    ui::UiText,
};
use crate::entity::dialog::talk_dialog::TalkDialog;
use crate::utils::resolve;

#[derive(SystemDesc)]
pub struct TalkDialogSystem {
    index: usize,
    char_index: usize,
    counter: usize,
    speed: f32,
    capframes: f32,
    hold: bool
}

impl TalkDialogSystem {
    pub fn new() -> Self {
        let mut system = TalkDialogSystem {
            index: 0,
            char_index: 0,
            counter: 0,
            speed: 15.0,
            capframes: 0.0,
            hold: false
        };

        system.capframes = resolve::get_fps() as f32 / system.speed;
        system
    }

    fn set_text(&self, ui_text: &mut UiText, dialog: &TalkDialog) {
        ui_text.text = dialog.text[self.index.clone()][0..self.char_index.clone() + 1].to_string();
    }

}

impl<'s> System<'s> for TalkDialogSystem {
    type SystemData = (
        WriteStorage<'s, TalkDialog>,
        WriteStorage<'s, UiText>,
        Read<'s, InputHandler<StringBindings>>
    );

    fn run(&mut self, (mut dialogs, mut ui_texts, input): Self::SystemData) {
        let should_continue = input.action_is_down("continue").unwrap_or(false);

        if should_continue {
            self.hold = true;
        }

        for (dialog, ui_text) in (&mut dialogs, &mut ui_texts).join() {

            if self.counter == self.capframes as usize {
                if self.char_index < dialog.text[self.index.clone()].len() - 1 {
                    self.char_index += 1;
                }
                self.counter = 0;
            }

            self.counter += 1;

            if self.hold && !should_continue {
                self.hold = false;

                self.char_index = 0;
                if self.index < dialog.text.len() - 1 {
                    self.index += 1;
                }
            }

            self.set_text(ui_text, dialog);
        }
    }
}