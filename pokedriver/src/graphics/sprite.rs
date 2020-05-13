use crate::utils::path;

use ggez::{graphics, Context, GameResult};


pub struct SpriteVector {
    pub data: Vec<graphics::Image>
}

impl SpriteVector {
    pub fn from(ctx: &mut Context, sprite_path: &String, n_frames: &u32) -> GameResult<SpriteVector> {
        let mut sprites = SpriteVector::new();

        for i in 0..n_frames - 1 {
            let image = graphics::Image::new(ctx, format!("{}/{}.png", sprite_path, i.to_string()))?;

            sprites.data.push(image);
        }

        Ok(sprites)
    }

    pub fn new() -> SpriteVector {
        SpriteVector { data: vec![] }
    }
}


// PokeSprites are merely a wrapper around sprite-vector
// and act as an interface between the sprite-vector
// and the game engine.

pub enum PokeSpriteType {
    NormalFront,
    NormalBack,
    ShinyFront,
    ShinyBack,
}

pub struct PokeSprite {
    sprite_vec: SpriteVector
}

impl PokeSprite {
    //todo: implement fn draw

    pub fn from(ctx: &mut Context, pokemon: &String, sprite_type: PokeSpriteType) -> GameResult<PokeSprite> {
        //todo: fetch frame count from resources.

        let frames: u32 = 10;

        let sprite_vec_path = &path::resolve_sprite_path(pokemon, sprite_type);

        let sprite = PokeSprite {
            sprite_vec: SpriteVector::from(ctx, sprite_vec_path, &frames)?,
        };

        Ok(sprite)
    }
}


