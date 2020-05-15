use cgmath::Point2;
use ggez::{Context, GameResult, graphics};
use ggez::graphics::DrawParam;
use crate::utils::resolver;
use crate::graphics::sprite_sync::SpriteSync;

pub struct SpriteVector {
    pub data: Vec<graphics::Image>
}

impl SpriteVector {
    pub fn from(ctx: &mut Context, sprite_path: &String, n_frames: &u16) -> GameResult<SpriteVector> {
        let mut sprites = SpriteVector::new();

        for i in 0..*n_frames {
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

pub enum PokemonSpriteType {
    NormalFront,
    NormalBack,
    ShinyFront,
    ShinyBack,
}

pub struct PokemonSprite {
    sprite_vec: SpriteVector,
    sync: SpriteSync,
}

impl PokemonSprite {
    pub fn new() -> PokemonSprite {
        PokemonSprite {
            sprite_vec: SpriteVector::new(),
            sync: SpriteSync::new(),
        }
    }

    pub fn from(ctx: &mut Context, pokemon: &String, sprite_type: &PokemonSpriteType) -> GameResult<PokemonSprite> {
        let sprite_vec_path = resolver::get_sprite_path(pokemon, sprite_type);
        let frames: u16 = resolver::get_anim_frames(ctx, pokemon, sprite_type)?;
        println!("loaded sprite-vector with frame-count: {}", frames);

        let sprite = PokemonSprite {
            sprite_vec: SpriteVector::from(ctx, &sprite_vec_path, &frames)?,
            sync: SpriteSync::new().set_frames(frames),
        };

        Ok(sprite)
    }

    pub fn draw(&mut self, ctx: &mut Context, pt: Point2<f32>) -> GameResult<()> {
        graphics::draw(ctx, &self.sprite_vec.data[self.sync.get_frame()], DrawParam::new().dest(pt))?;
        self.sync.update();

        Ok(())
    }
}


