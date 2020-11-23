use amethyst::{
    prelude::*,
    core::{
        transform::Transform,
        math::{Vector2, Vector3},
    },
    ecs::{Component, DenseVecStorage},
    renderer::{SpriteSheet, SpriteRender},
    assets::Handle,
};

use crate::utils::resolve;
use crate::entity::actor::ActorAttrs;


#[derive(Clone)]
pub struct Player {
    pub attrs: ActorAttrs,
    pub sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    pub grid_pos: Vector2<i32>
}

impl Player {
    pub fn create(world: &mut World, name: String) {
        let sprite_sheet_handle = resolve::load_spritesheet_handle(world, "players/".to_string() + name.as_str());
        let mut player = Player {
            attrs: ActorAttrs::new(),
            sprite_sheet_handle: Some(sprite_sheet_handle),
            grid_pos: Vector2::new(0, 0)
        };

        player.init(world);
    }

    fn init(&mut self, world: &mut World) {
        if let Some(sprite_sheet_handle) = self.sprite_sheet_handle.clone() {
            let sprite = SpriteRender {
                sprite_sheet: sprite_sheet_handle,
                sprite_number: self.attrs.to_sprite_index(),
            };

            let mut transform = Transform::default();
            transform.set_translation_xyz(320.0, 320.0, 2.0);
            transform.set_scale(Vector3::new(2.0, 2.0, 1.0));

            world.create_entity()
                .with(sprite)
                .with(self.clone())
                .with(transform)
                .build();
        }
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

impl Default for Player {
    fn default() -> Self {
        Player {
            attrs: ActorAttrs::new(),
            sprite_sheet_handle: None,
            grid_pos: Vector2::new(0, 0)
        }
    }
}

