use crate::graphics::sprite::PokeSpriteType;
use crate::graphics::tile::PokeTileTypes;

pub fn sprite_path(pokemon: &String, sprite_type: PokeSpriteType) -> String {
    match sprite_type {
        PokeSpriteType::NormalFront => format!("sprites/pokemon/normal-front/{}", pokemon),
        PokeSpriteType::NormalBack => format!("sprites/pokemon/normal-back/{}", pokemon),
        PokeSpriteType::ShinyFront => format!("sprites/pokemon/shiny-front/{}", pokemon),
        PokeSpriteType::ShinyBack => format!("sprites/pokemon/shiny-back/{}", pokemon)
    }
}

pub fn tile_path(tile_type: PokeTileTypes) -> String {
    match tile_type {
        PokeTileTypes::GreenPatch => String::from("/tiles/GreenPatch-1-16x16.png")
    }
}

#[inline]
pub fn get_fps() -> u16 {
    60
}


