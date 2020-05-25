use ggez::{Context, GameResult};
use std::cell::RefCell;
use cgmath::Point2;
use crate::engine::engine::SharedState;
use crate::graphics::overworld::ViewPort;
use crate::graphics::components::ComponentIdentity;

pub mod sprite;
pub mod fsync;
pub mod tile;
pub mod actor;
pub mod overworld;
pub mod components;
pub mod dialog;

pub trait Component {
    fn update(&mut self, state: &RefCell<SharedState>) -> GameResult<()>;
    fn draw(&mut self, ctx: &mut Context, view_port: &ViewPort) -> GameResult<()>;
    fn location(&self) -> Point2<f32>;
    fn id(&self) -> ComponentIdentity;
}