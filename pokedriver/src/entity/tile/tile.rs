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

use crate::entity::tile::{TileAttrs, TileClass};
use crate::utils::resolve;


pub struct Tile {
    attrs: TileAttrs,
    sheet: Handle<SpriteSheet>,
}

impl Tile {
    pub fn new(world: &mut World, class: TileClass) -> Self {
        let class_str = "tiles/".to_string() + class.to_string().as_str();
        let sheet = resolve::load_spritesheet_handle(world, class_str);
        let mut tile = Tile {
            sheet,
            attrs: TileAttrs {
                class,
                state: 0,
            },
        };

        tile.init(world);
        tile
    }

    fn init(&mut self, world: &mut World) {
        let sprite = SpriteRender {
            sprite_sheet: self.sheet.clone(),
            sprite_number: self.attrs.to_sprite_index(),
        };

        let mut transform = Transform::default();
        transform.set_translation_xyz(320.0, 320.0, 1.0);
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

