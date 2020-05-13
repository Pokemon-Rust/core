use ggez::{Context, GameResult, graphics};

use crate::utils::resolver;
use cgmath::Point2;
use ggez::graphics::DrawParam;
use ggez::filesystem;
use std::path::Path;
use std::io::Read;
use serde_json::Value;

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

pub enum PokeSpriteType {
    NormalFront,
    NormalBack,
    ShinyFront,
    ShinyBack,
}

pub struct PokeSprite {
    sprite_vec: SpriteVector,
    frame_id: f32,
    n_frames: u16,
    event_loop_frame_id: u16
}

impl PokeSprite {
    pub fn from(ctx: &mut Context, pokemon: &String, sprite_type: &PokeSpriteType) -> GameResult<PokeSprite> {
        let sprite_vec_path = resolver::get_sprite_path(pokemon, sprite_type);
        let frames: u16 = resolver::get_anim_frames(ctx, pokemon, sprite_type)?;
        println!("loaded sprite-vector with frame-count: {}", frames);

        let sprite = PokeSprite {
            sprite_vec: SpriteVector::from(ctx, &sprite_vec_path, &frames)?,
            frame_id: 0.0,
            event_loop_frame_id: 0,
            n_frames: frames,
        };

        Ok(sprite)
    }

    pub fn draw(&mut self, ctx: &mut Context, pt: Point2<f32>) -> GameResult<()> {
        graphics::draw(ctx, &self.sprite_vec.data[self.frame_id.floor() as usize], DrawParam::new().dest(pt))?;

        let desired_fps = resolver::get_fps();

        if self.event_loop_frame_id == desired_fps - 1 {
            self.event_loop_frame_id = 0;
        } else {
            self.event_loop_frame_id += 1;
        }

        if self.frame_id.ceil() as u16 >= self.n_frames - 1 {
            self.frame_id = 0.0;
        } else {
            if self.n_frames < desired_fps {
                self.frame_id = ((self.event_loop_frame_id as f32) * (self.n_frames as f32)) / (desired_fps as f32);
            } else {
                self.frame_id += 1.0;
            }
        }
        Ok(())
    }
}


