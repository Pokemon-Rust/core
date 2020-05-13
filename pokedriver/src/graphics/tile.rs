use ggez::{Context, GameResult, graphics};
use cgmath::{Point2};

use crate::utils::resolve;
use ggez::graphics::DrawParam;

pub struct PokeTile {
    tile: graphics::Image
}

impl PokeTile {
    pub fn from(ctx: &mut Context, tile_type: &PokeTileTypes) -> GameResult<PokeTile> {
        let path = resolve::tile_path(tile_type);
        let tile = PokeTile { tile: graphics::Image::new(ctx, path)? };
        Ok(tile)
    }

    pub fn draw(&self, ctx: &mut Context, pt: Point2<f32>) -> GameResult<()> {
        graphics::draw(ctx, &self.tile, DrawParam::new().dest(pt))
    }
}

pub enum PokeTileTypes {
    GreenPatch
}