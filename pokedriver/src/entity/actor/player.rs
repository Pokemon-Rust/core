use amethyst::{
    prelude::*,
    core::{transform::{Transform, TransformBundle}},
    ecs::{Entity, Component, DenseVecStorage},
    renderer::{Texture, SpriteSheet, SpriteRender, ImageFormat, Transparent},
    assets::{AssetStorage, Handle, Loader},
};

use crate::utils::resolve;

#[derive(Clone)]
pub struct Player {
    sprite_index: usize,
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    speed: f32,
}

impl Player {
    pub fn new(world: &mut World, name: String, speed: f32) -> Entity {
        let sprite_sheet_handle = resolve::load_spritesheet_handle(world, name);

        let player = Player {
            sprite_index: 0,
            sprite_sheet_handle: Some(sprite_sheet_handle.clone()),
            speed,
        };

        let sprite = SpriteRender {
            sprite_sheet: sprite_sheet_handle.clone(),
            sprite_number: 0,
        };

        let mut transform = Transform::default();
        transform.set_translation_xyz(320.0, 240.0, 1.0);

        world.create_entity()
            .with(sprite)
            .with(player.clone())
            .with(transform)
            .build()
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}