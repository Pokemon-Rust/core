use crate::graphics::sprite::PokeSpriteType;

pub fn resolve_sprite_path(pokemon: &String, sprite_type: PokeSpriteType) -> String {
    match sprite_type {
        PokeSpriteType::NormalFront => format!("sprites/{}/normal-front", pokemon),
        PokeSpriteType::NormalBack => format!("sprites/{}/normal-back", pokemon),
        PokeSpriteType::ShinyFront => format!("sprites/{}/shiny-front", pokemon),
        PokeSpriteType::ShinyBack => format!("sprites/{}/shiny-back", pokemon)
    }
}


