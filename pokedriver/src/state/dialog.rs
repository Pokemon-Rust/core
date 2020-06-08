use amethyst::{
    prelude::*,
    window::ScreenDimensions,
    core::{transform::Transform},
    renderer::camera::{Camera, Projection},
};

use crate::entity::actor::player::Player;
use crate::utils::debug;
use crate::entity::tile::{tile::Tile};


pub struct DialogState {

}

impl DialogState {
    pub fn new() -> Self {
        DialogState {}
    }



}

impl SimpleState for DialogState {

}