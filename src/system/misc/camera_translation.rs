use amethyst::{
    prelude::*,
    derive::SystemDesc,
    ecs::prelude::{Join, System, SystemData, WriteStorage, Write},
    renderer::{
        camera::Camera
    },
    core::transform::Transform,
};

use crate::state::Game;

#[derive(SystemDesc)]
pub struct CameraTranslationSystem;

impl CameraTranslationSystem {
    pub fn new() -> Self {
        CameraTranslationSystem
    }
}

impl<'s> System<'s> for CameraTranslationSystem {
    type SystemData = (
        WriteStorage<'s, Camera>,
        WriteStorage<'s, Transform>,
        Write<'s, Game>
    );

    fn run(&mut self, (mut cameras, mut transforms, mut game): Self::SystemData) {
        for (_, transform) in (&mut cameras, &mut transforms).join() {
            game.camera_trans = transform.translation().clone();
        }
    }
}