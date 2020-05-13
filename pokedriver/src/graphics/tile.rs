use ggez::{Context, GameResult, graphics};
use cgmath::{Point2};

use crate::utils::resolver;
use ggez::graphics::DrawParam;

pub struct Tile {
    tile: graphics::Image
}

impl Tile {
    pub fn from(ctx: &mut Context, tile_type: &TileType) -> GameResult<Tile> {
        let path = resolver::get_tile_path(tile_type);
        let tile = Tile { tile: graphics::Image::new(ctx, path)? };
        Ok(tile)
    }

    pub fn draw(&self, ctx: &mut Context, pt: Point2<f32>) -> GameResult<()> {
        graphics::draw(ctx, &self.tile, DrawParam::new().dest(pt))
    }
}

pub enum TileType {
    GreenPatch
}