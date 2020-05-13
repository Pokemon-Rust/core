use ggez::{ContextBuilder, filesystem, GameResult, graphics};
use std::path;
use pokedriver::graphics as pokegraphics;

fn test_sprite_load() -> GameResult {
    let mut cb = ContextBuilder::new("pokemon", "vishaal");
    let mut path = path::PathBuf::from("./");
    path.push("resources");
    cb = cb.add_resource_path(path);

    let (ctx, _) = &mut cb.build()?;

    let sprites = pokegraphics::sprite::SpriteVector::from(ctx, &"pikachu".to_owned());

    Ok (())
}

fn main() {
    println!("Hello, pokemon world!");
    test_sprite_load().expect("error occured ...");
}