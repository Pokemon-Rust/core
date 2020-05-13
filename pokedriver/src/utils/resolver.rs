use std::path::Path;
use ggez::{Context, filesystem, GameError, GameResult};
use serde_json::Value;

use crate::graphics::sprite::PokeSpriteType;
use crate::graphics::tile::PokeTileTypes;
use crate::utils::resolver;


pub fn get_sprite_path(pokemon: &String, sprite_type: &PokeSpriteType) -> String {
    match sprite_type {
        PokeSpriteType::NormalFront => format!("sprites/pokemon/normal-front/{}", pokemon),
        PokeSpriteType::NormalBack => format!("sprites/pokemon/normal-back/{}", pokemon),
        PokeSpriteType::ShinyFront => format!("sprites/pokemon/shiny-front/{}", pokemon),
        PokeSpriteType::ShinyBack => format!("sprites/pokemon/shiny-back/{}", pokemon)
    }
}

pub fn get_tile_path(tile_type: &PokeTileTypes) -> String {
    match tile_type {
        PokeTileTypes::GreenPatch => String::from("/tiles/GreenPatch-1-16x16.png")
    }
}

pub fn get_anim_frames(ctx: &mut Context, pokemon: &String, sprite_type: &PokeSpriteType) -> GameResult<u16> {
    let json_file = Path::new("/spritedata/framecount.json");
    let file = filesystem::open(ctx, json_file)?;
    let v: Value = match serde_json::from_reader(file) {
        Ok(file) => file,
        Err(_) => return Err(GameError::ResourceLoadError("Error reading JSON from file".to_string()))
    };
    let sprite_vec_path = resolver::get_sprite_path(pokemon, sprite_type);
    let frames: u16 = match v[sprite_vec_path.clone()].as_u64() {
        Some(f) => f as u16,
        None => return Err(GameError::ResourceLoadError("Error parsing JSON from file".to_string()))
    };
    Ok(frames)
}

#[inline]
pub fn get_fps() -> u16 {
    60
}


