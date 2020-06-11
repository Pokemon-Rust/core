use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, System, SystemData, Write, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::{SpriteRender, camera::Camera},
    core::math::Vector3
};
use crate::entity::actor::player::Player;
use crate::state::Game;

// The run() function returns a boolean value stating whether the behaviour corresponded to the input.
pub trait PlayerBehaviour {
    fn run(&mut self, player: &mut Player, transform: &mut Transform, input: &Read<InputHandler<StringBindings>>) -> bool;
}


#[derive(SystemDesc)]
pub struct PlayerSystem {
    behaviours: Vec<Box<dyn PlayerBehaviour + Send + Sync>>
}

impl PlayerSystem {
    pub fn new() -> Self {
        PlayerSystem {
            behaviours: Vec::new()
        }
    }

    pub fn add_behaviour(&mut self, behaviour: Box<dyn PlayerBehaviour + Send + Sync>) {
        self.behaviours.push(behaviour);
    }

    fn draw(&mut self, player: &Player, sprite_render: &mut SpriteRender) {
        sprite_render.sprite_number = player.attrs.to_sprite_index();
    }
}

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        WriteStorage<'s, Player>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Camera>,
        Read<'s, InputHandler<StringBindings>>,
        Write<'s, Game>
    );

    fn run(&mut self, (mut players, mut sprites, mut transforms, mut cameras, input, mut game): Self::SystemData) {
        let mut translation: Vector3<f32> = Vector3::new(0.0, 0.0, 2.0);

        for (transform, _camera) in (&mut transforms, &mut cameras).join() {
            translation = transform.translation().clone();
            for (player, sprite) in (&mut players, &mut sprites).join() {
                for behaviour in &mut self.behaviours {

                    // If the input was handled by a behaviour, skip all other behaviours.
                    // Only one behaviour is allowed to run at a time.
                    if behaviour.run(player, transform, &input) {
                        break;
                    }
                }

                self.draw(player, sprite);
            }
        }

        for (_player, player_transform) in (&mut players, &mut transforms).join() {
            player_transform.set_translation_x(translation[0] + 320.0);
            player_transform.set_translation_y(translation[1] - 320.0);
        }
    }
}