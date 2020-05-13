use std::path::Path;
use ggez::{Context, filesystem, GameError, GameResult};
use serde_json::Value;

use crate::graphics::sprite::PokemonSpriteType;
use crate::graphics::tile::TileType;
use crate::utils::resolver;


pub fn get_sprite_path(pokemon: &String, sprite_type: &PokemonSpriteType) -> String {
    match sprite_type {
        PokemonSpriteType::NormalFront => format!("sprites/pokemon/normal-front/{}", pokemon),
        PokemonSpriteType::NormalBack => format!("sprites/pokemon/normal-back/{}", pokemon),
        PokemonSpriteType::ShinyFront => format!("sprites/pokemon/shiny-front/{}", pokemon),
        PokemonSpriteType::ShinyBack => format!("sprites/pokemon/shiny-back/{}", pokemon)
    }
}

pub fn get_tile_path(tile_type: &TileType) -> String {
    match tile_type {
        TileType::GreenPatch => String::from("/tiles/GreenPatch-1-16x16.png")
    }
}

pub fn get_anim_frames(ctx: &mut Context, pokemon: &String, sprite_type: &PokemonSpriteType) -> GameResult<u16> {
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


