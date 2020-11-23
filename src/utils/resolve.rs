use amethyst::{
    prelude::*,
    renderer::{Texture, SpriteSheet, ImageFormat, SpriteSheetFormat},
    assets::{AssetStorage, Handle, Loader},
    ui::{TtfFormat, FontAsset},
};


pub fn load_texture_handle(world: &mut World, name: String) -> Handle<Texture> {
    let base_str = "texture/";
    let pic_path = base_str.to_string() + name.as_ref() + ".png".to_string().as_ref();

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

    texture_handle
}

pub fn load_spritesheet_handle(world: &mut World, name: String) -> Handle<SpriteSheet> {
    let base_str = "texture/";
    let ron_path = base_str.to_string() + name.as_ref() + ".ron";

    let texture_handle = load_texture_handle(world, name);

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        ron_path, // Here we load the associated ron file
        SpriteSheetFormat(texture_handle), // We pass it the texture we want it to use
        (),
        &sprite_sheet_store,
    )
}


pub fn load_font_handle(world: &mut World) -> Handle<FontAsset> {
    let font_storage = world.read_resource::<AssetStorage<FontAsset>>();
    world.read_resource::<Loader>().load("font/pokemon_fire_red.ttf",
                                         TtfFormat,
                                         (),
                                         &font_storage)
}

pub fn get_fps() -> usize {
    60
}