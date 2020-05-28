use amethyst::{
    prelude::*,
    ecs::{Component, DenseVecStorage},
    renderer::{Texture, SpriteSheet, ImageFormat},
    assets::{AssetStorage, Handle, Loader}
};

pub struct Player {
    sprite_index: usize,
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    speed: f32
}

impl Player {
    pub fn new(world: &mut World, name: String, speed: f32) -> Self {
        let base_str = "texture/";
        let pic_path = base_str + name.as_str() + ".png";
        let ron_path = base_str + name.as_str() + ".ron";

        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                pic_path,
                ImageFormat::default(),
                (),
                &texture_storage,
            )
        };

        let loader = world.read_resource::<Loader>();
        let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
        let sprite_sheet_handle = loader.load(
            ron_path, // Here we load the associated ron file
            SpriteSheetFormat(texture_handle), // We pass it the texture we want it to use
            (),
            &sprite_sheet_store,
        );

        Player {
            sprite_index: 0,
            sprite_sheet_handle: Some(sprite_sheet_handle),
            speed
        }
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}