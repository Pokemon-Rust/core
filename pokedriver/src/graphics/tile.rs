use ggez::{Context, GameResult, graphics};
use cgmath::{Point2, Vector2};

use crate::utils::resolver;
use ggez::graphics::DrawParam;
use crate::graphics::Component;
use std::cell::RefCell;
use crate::engine::engine::SharedState;
use crate::graphics::overworld::ViewPort;
use crate::graphics::components::ComponentIdentity;

pub struct Tile {
    tile: graphics::Image,
    location: Point2<f32>,
    tile_type: TileType
}

impl Tile {
    pub fn from(ctx: &mut Context, tile_type: &TileType, location: Point2<f32>) -> GameResult<Tile> {
        let path = resolver::get_tile_path(tile_type);
        let tile = Tile {
            tile: graphics::Image::new(ctx, path)?,
            tile_type: tile_type.clone(),
            location,
        };

        Ok(tile)
    }
}

impl Component for Tile {
    fn update(&mut self, _state: &RefCell<SharedState>) -> GameResult<()> {
        // todo: support dynamic tiles.
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, view_port: &ViewPort) -> GameResult<()> {
        let scale_vec = Vector2 {
            x: view_port.scale_x,
            y: view_port.scale_y,
        };

        graphics::draw(ctx, &self.tile, DrawParam::new()
            .dest(view_port.translate(self.location))
            .scale(scale_vec))
    }

    fn location(&self) -> Point2<f32> {
        self.location
    }

    fn id(&self) -> ComponentIdentity {
        ComponentIdentity::Tile(self.tile_type)
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum TileType {
    GreenPatch
}