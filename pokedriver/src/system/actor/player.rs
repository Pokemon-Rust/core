use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::{SpriteRender, camera::Camera},
};

use crate::entity::actor::player::Player;

// The run() function returns a boolean value stating whether the behaviour corresponded to the input.
pub trait PlayerBehaviour {
    fn run(&mut self, player: &mut Player, camera: &mut Camera, input: &Read<InputHandler<StringBindings>>) -> bool;
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
    );

    fn run(&mut self, (mut players, mut sprites, mut transforms, mut cameras, input): Self::SystemData) {
        for (_cam_transform, camera) in (&mut transforms, &mut cameras).join() {
            for (player, sprite) in (&mut players, &mut sprites).join() {
                for behaviour in &mut self.behaviours {

                    // If the input was handled by a behaviour, skip all other behaviours.
                    // Only one behaviour is allowed to run at a time.
                    if behaviour.run(player, camera, &input) {
                        break;
                    }
                }

                self.draw(player, sprite);
            }
        }

    }
}