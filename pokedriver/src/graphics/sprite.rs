use crate::utils::path;

use ggez::{graphics, Context};


pub struct SpriteVector {
    pub data: Vec<graphics::Image>
}

impl SpriteVector {
    pub fn from(ctx: &mut Context, pokemon: &String) -> SpriteVector {
        let mut sprites = SpriteVector::new();
        for i in 0..32 {
            let image = graphics::Image::new(ctx, path::resolve_sprite_path(pokemon) +
                i.to_string().as_ref() + ".png").expect("error");
            sprites.data.push(image);
        }

        return sprites;
    }

    pub fn new() -> SpriteVector {
        SpriteVector { data: vec![] }
    }
}


pub enum SpriteVectorResult {
    Normal(SpriteVector),
    Back(SpriteVector),
    Shiny(SpriteVector),
    ShinyBack(SpriteVector),
}
