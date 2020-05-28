pub mod game;
pub mod actor;

use amethyst::{
    prelude::*,
    ecs::Entity
};

pub trait DrawableEntity {
    fn entity(&self) -> Option<Entity>;
    fn draw(&mut self, world: &mut World);
}