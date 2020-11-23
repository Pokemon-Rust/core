use amethyst::{
    prelude::*,
    core::{
        transform::Transform,
        math::Vector3,
    },
    ecs::{Component, DenseVecStorage},
    renderer::{SpriteSheet, SpriteRender},
    assets::Handle,
};

use crate::entity::tile::{TileAttrs};
use crate::utils::resolve;

#[derive(Clone)]
pub struct Tile {
    pub attrs: TileAttrs,
    pub sheet: Handle<SpriteSheet>,
}

impl Tile {
    pub fn create(world: &mut World, tileset: String) {
        let tileset_str = "tiles/".to_string() + tileset.to_string().as_str();
        let sheet = resolve::load_spritesheet_handle(world, tileset_str);
        let mut tile = Tile {
            sheet,
            attrs: TileAttrs {
                class: None,
                state: 0,
            },
        };

        tile.init(world);
    }

    fn init(&mut self, world: &mut World) {
        let sprite = SpriteRender {
            sprite_sheet: self.sheet.clone(),
            sprite_number: self.attrs.to_sprite_index(),
        };

        let mut transform = Transform::default();
        transform.set_translation_xyz(320.0, 296.0, 1.0);
        transform.set_scale(Vector3::new(2.0, 2.0, 1.0));

        world.create_entity()
            .with(sprite)
            .with(self.clone())
            .with(transform)
            .build();
    }
}

impl Component for Tile {
    type Storage = DenseVecStorage<Self>;
}

