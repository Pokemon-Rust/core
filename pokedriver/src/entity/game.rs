use amethyst::{
    prelude::*,
    window::ScreenDimensions,
    core::{transform::Transform},
    renderer::camera::{Camera, Projection},
    ecs::Entity,
};

use crate::entity::actor::player;
use crate::entity::actor::player::Player;
use crate::entity::DrawableEntity;
use std::borrow::BorrowMut;

pub struct GameState {
    camera: Option<Entity>,
    drawables: Vec<Box<dyn DrawableEntity>>
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            camera: None,
            drawables: Vec::new()
        }
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

        let camera = world.create_entity()
            .with(camera)
            .with(transform)
            .build();

        self.camera = Some(camera);
    }

    fn initialize_player(&mut self, world: &mut World) {
        let player = player::Player::new(world, "boy".to_string(), 2.0);
        self.drawables.push(Box::new(player));
    }
}

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world= data.world;
        self.initialize_camera(world);
        self.initialize_player(world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let world = data.world.borrow_mut();

        for drawable in &mut self.drawables {
            if let Some(entity) = drawable.entity() {
                world.delete_entity(entity).expect("Error occured while deletion");
                drawable.draw(world);
            }
        }

        Trans::None
    }
}