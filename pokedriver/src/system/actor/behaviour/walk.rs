use amethyst::{
    core::{timing::Time, transform::Transform},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, Write, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::{SpriteRender, camera::Camera},
};

use crate::system::actor::player::PlayerBehaviour;
use crate::entity::actor::player::Player;
use crate::entity::actor::{ActorAttrs, ActorDirection, ActorAction};


pub struct Walk {

}

impl Walk {
    pub fn new() -> Box<Self> {
        Box::new(Walk {})
    }
}

impl PlayerBehaviour for Walk {
    fn run(&mut self, player: &mut Player, sprite: &mut SpriteRender, camera: &mut Camera, input: &Read<InputHandler<StringBindings>>) {
        player.attrs = ActorAttrs {
            direction: ActorDirection::North,
            action: ActorAction::Stand,
        };
        self.draw(player, sprite);
    }

    fn draw(&mut self, player: &Player, sprite_render: &mut SpriteRender) {
        sprite_render.sprite_number = player.attrs.to_sprite_index();
    }
}