use ggez::{Context, GameResult};
use std::cell::RefCell;
use cgmath::Point2;
use crate::engine::engine::SharedState;
use crate::graphics::overworld::ViewPort;

pub mod sprite;
mod sprite_sync;
pub mod tile;
pub mod actor;
pub mod overworld;

pub trait Renderable {
    fn update(&mut self, state: &RefCell<SharedState>) -> GameResult<()>;
    fn draw(&mut self, ctx: &mut Context, view_port: &ViewPort) -> GameResult<()>;
    fn location(&self) -> Point2<f32>;
}