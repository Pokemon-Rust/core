use amethyst::{
    prelude::*,
    core::{
        transform::Transform,
        math::Vector3,
    },
    ecs::{Component, DenseVecStorage, Entity},
    ui::{UiTransform, UiImage, UiText, Anchor, LineMode},
    renderer::{SpriteRender, Transparent},
    assets::Handle,
};

use crate::utils::resolve;
use crate::entity::dialog::DialogSpritetype;


#[derive(Clone)]
pub struct TalkDialog {
    pub text: Vec<String>,
    pub index: usize,
    pub char_index: usize,
    pub mesh: Option<Entity>
}

impl TalkDialog {
    pub fn create(world: &mut World) {
        let mut dialog = TalkDialog {
            text: Vec::new(),
            index: 0,
            char_index: 0,
            mesh: None,
        };

        dialog.text.push("Hello, I'm Professor Oak.\n\nWelcome to the world of Pokemon!".to_string());
        dialog.text.push("Your objective is simple.\n\nScrew your rival and the elite four and get to the hall of fame.".to_string());

        dialog.init(world);
    }

    fn init(&mut self, world: &mut World) {
        let font = resolve::load_font_handle(world);

        let mut text = UiText::new(
            font,
            "".to_string(),
            [1., 1., 1., 1.],
            32.,
        );

        text.line_mode = LineMode::Wrap;
        text.align = Anchor::TopLeft;

        let sprite_handle = resolve::load_spritesheet_handle(world, "dialogs/sheet".to_string());
        let sprite_render = SpriteRender {
            sprite_sheet: sprite_handle,
            sprite_number: DialogSpritetype::Bottom.to_sprite_index()
        };

        let mut transform = Transform::default();
        transform.set_translation_xyz(320.0, 80.0, 3.0);
        transform.set_scale(Vector3::new(1.2,1.1,1.0));

        let text_transform = UiTransform::new(
            "dialog_bottom_text".to_string(),
            Anchor::BottomLeft,
            Anchor::MiddleLeft,
            24.,
            64.,
            4.,
            592.,
            144.
        );

        self.mesh = Some(world.create_entity()
            .with(transform.clone())
            .with(sprite_render)
            .with(Transparent)
            .build());

        world.create_entity()
            .with(text)
            .with(text_transform)
            .with(self.clone())
            .build();
    }
}


impl Component for TalkDialog {
    type Storage = DenseVecStorage<Self>;
}
