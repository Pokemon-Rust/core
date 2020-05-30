use amethyst::{
    core::{timing::Time, transform::Transform},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, Write, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::{SpriteRender, camera::Camera},
};

use crate::entity::actor::player::Player;
use crate::entity::actor::{ActorAttrs, ActorAction, ActorDirection};

pub trait PlayerBehaviour {
    fn run(&mut self, player: &mut Player, sprite: &mut SpriteRender, camera: &mut Camera, input: &Read<InputHandler<StringBindings>>);
    fn draw(&mut self, player: &Player, sprite_render: &mut SpriteRender);
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
        for (cam_transform, camera) in (&mut transforms, &mut cameras).join() {
            for (player, sprite) in (&mut players, &mut sprites).join() {
                for behaviour in &mut self.behaviours {
                    behaviour.run(player, sprite, camera, &input);
                }
                // player.attrs = ActorAttrs {
                //     direction: ActorDirection::North,
                //     action: ActorAction::Stand,
                // };
                // self.draw(player, sprite);
            }
        }
    }
}