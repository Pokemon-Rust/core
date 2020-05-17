use ggez::{Context, GameResult, graphics};
use cgmath::{Point2};

use crate::utils::resolver;
use ggez::graphics::DrawParam;
use crate::graphics::Renderable;
use std::cell::RefCell;
use crate::engine::engine::SharedState;
use crate::graphics::overworld::ViewPort;

pub struct Tile {
    tile: graphics::Image,
    location: Point2<f32>,
}

impl Tile {
    pub fn from(ctx: &mut Context, tile_type: &TileType, location: Point2<f32>) -> GameResult<Tile> {
        let path = resolver::get_tile_path(tile_type);
        let tile = Tile {
            tile: graphics::Image::new(ctx, path)?,
            location,
        };

        Ok(tile)
    }
}

impl Renderable for Tile {
    fn update(&mut self, state: &RefCell<SharedState>) -> GameResult<()> {
        // todo: support dynamic tiles.
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, view_port: &ViewPort) -> GameResult<()> {
        graphics::draw(ctx, &self.tile, DrawParam::new().dest(view_port.translate(self.location)))
    }

    fn location(&self) -> Point2<f32> {
        self.location
    }
}

pub enum TileType {
    GreenPatch
}