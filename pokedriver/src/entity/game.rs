use amethyst::{
    prelude::*,
    window::ScreenDimensions,
    core::{transform::Transform},
    renderer::camera::{Camera, Projection},
    ecs::Entity,
};

pub struct GameState {
    camera: Option<Entity>
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            camera: None
        }
    }

    fn initialize_camera(&mut self, world: &mut World) {
        let (width, height) = {
            let dim = world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };

        let mut transform = Transform::default();
        transform.set_translation_xyz(width / 2.0, height / 2.0, 1.0);

        let mut camera = Camera::standard_3d(width, height);
        camera.set_projection(Projection::orthographic(0.0, width, 0.0, height, 0.0, 20.0));

        let camera = world.create_entity()
            .with(transform)
            .with(camera)
            .build();

        self.camera = Some(camera);
    }
}

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        self.initialize_camera(world);
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::None
    }
}