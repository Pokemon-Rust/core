use amethyst::{
    core::{timing::Time, transform::Transform},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::{SpriteRender},
};

use crate::entity::actor::player::Player;
use crate::entity::actor::{ActorAttrs, ActorAction, ActorDirection};

#[derive(SystemDesc)]
pub struct PlayerSystem {
    counter: usize
}

impl PlayerSystem {
    pub fn new() -> Self {
        PlayerSystem {
            counter: 0
        }
    }

    fn draw(&mut self, player: &Player, sprite_render: &mut SpriteRender) {
        sprite_render.sprite_number = player.attrs.to_sprite_index();
    }
}

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        WriteStorage<'s, Player>,
        WriteStorage<'s, SpriteRender>,
        ReadStorage<'s, Transform>
    );

    fn run(&mut self, (mut players, mut sprites, _transforms): Self::SystemData) {
        for (player, sprite) in (&mut players, &mut sprites).join() {
            player.attrs = ActorAttrs {
                direction: ActorDirection::North,
                action: ActorAction::Stand,
            };
            self.draw(player, sprite);
        }
    }
}