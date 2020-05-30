use amethyst::{
    prelude::*,
    window::ScreenDimensions,
    core::{transform::Transform},
    renderer::camera::{Camera, Projection},
};

use crate::entity::actor::player::Player;

pub struct GameState {}

impl GameState {
    pub fn new() -> Self {
        GameState {}
    }

    fn initialize_camera(&mut self, world: &mut World) {
        let (width, height) = {
            let dim = world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };

        let mut transform = Transform::default();
        transform.set_translation_xyz(0.0, height, 10.0);

        let mut camera = Camera::standard_3d(width, height);
        camera.set_projection(Projection::orthographic(0.0, width, 0.0, height, 0.0, 20.0));

        world.create_entity()
            .with(camera)
            .with(transform)
            .build();
    }

    fn initialize_player(&mut self, world: &mut World) {
        Player::new(world, "nate".to_string(), 2.0);
    }
}

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        self.initialize_camera(world);
        self.initialize_player(world);
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::None
    }
}