use amethyst::{
    prelude::*,
    core::{transform::{Transform, TransformBundle}},
    ecs::{Entity, Component, DenseVecStorage},
    renderer::{Texture, SpriteSheet, SpriteRender, ImageFormat, Transparent},
    assets::{AssetStorage, Handle, Loader},
};

use crate::utils::resolve;
use crate::entity::DrawableEntity;

#[derive(Clone)]
pub struct Player {
    sprite_index: usize,
    sprite_sheet_handle: Handle<SpriteSheet>,
    speed: f32,
    entity: Option<Entity>
}

impl Player {
    pub fn new(world: &mut World, name: String, speed: f32) -> Player {
        let sprite_sheet_handle = resolve::load_spritesheet_handle(world, name);
        let mut player = Player {
            sprite_index: 0,
            sprite_sheet_handle: sprite_sheet_handle.clone(),
            speed,
            entity: None
        };
        player.draw(world);
        player
    }
}

impl DrawableEntity for Player {
    fn entity(&self) -> Option<Entity> {
        self.entity
    }

    fn draw(&mut self, world: &mut World) {
        let sprite = SpriteRender {
            sprite_sheet: self.sprite_sheet_handle.clone(),
            sprite_number: self.sprite_index,
        };

        let mut transform = Transform::default();
        transform.set_translation_xyz(320.0, 240.0, 1.0);

        let player = world.create_entity()
            .with(sprite)
            .with(self.clone())
            .with(transform)
            .build();

        self.entity = Some(player);
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}