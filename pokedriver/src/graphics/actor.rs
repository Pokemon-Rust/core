use std::cell::RefCell;
use std::collections::HashMap;

use cgmath::mint::Point2;
use cgmath::mint::Vector2;
use ggez::{Context, GameResult, graphics};
use ggez::graphics::DrawParam;

use crate::engine::engine::SharedState;
use crate::scripts::actor;
use crate::utils::resolver;

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum ActorDirection {
    North,
    South,
    East,
    West,
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum ActorAction {
    Stand,
    Walk1,
    Walk2,
    // todo: add more types for surf, running, bicycling etc.
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct ActorAttributes {
    pub direction: ActorDirection,
    pub action: ActorAction,
}


// The Actor struct encapsulates all the sprites corresponding to the actor.
// The script uses the SharedState to operate properly,
// i.e. Bound actor to TileMaps, etc. Finally, the script updates the actor attributes using
// &mut Actor. The actor::update() function executes the script at first, then the actor::draw() fn
// renders the sprite corresponding to the ActorAttribute specified by the script,
// ny changes to the storyline can be made
// from the script using a mutable reference to the SharedState.

pub struct Actor {
    pub attributes: ActorAttributes,
    pub location: Point2<f32>,

    sprite_map: HashMap<ActorAttributes, graphics::Image>,
    behaviour: Box<dyn actor::ActorBehaviour>,
}

impl Actor {
    pub fn from(ctx: &mut Context, actor: &String, attribute_batch: &Vec<ActorAttributes>, actor_behaviour_type: &actor::loader::ActorBehaviourType) -> GameResult<Actor> {
        let mut map = HashMap::new();

        for attribute in attribute_batch {
            let actor_path = resolver::get_actor_path(ctx, actor, attribute)?;
            map.insert(attribute.clone(), graphics::Image::new(ctx, actor_path)?);
        }

        let actor = Actor {
            attributes: ActorAttributes {
                direction: ActorDirection::South,
                action: ActorAction::Stand,
            },
            location: Point2 {
                x: 100.0,
                y: 100.0,
            },
            sprite_map: map.clone(),
            behaviour: actor::loader::load(actor_behaviour_type),
        };

        Ok(actor)
    }

    pub fn update(&mut self, _ctx: &mut Context, state: &RefCell<SharedState>) -> GameResult<()> {
        self.behaviour.run(&mut self.attributes, state)?;

        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        //todo: implement actor sprite rendering.
        let sprite = &self.sprite_map[&self.attributes];
        let (width, height) = graphics::drawable_size(ctx);
        let scale_vec = Vector2 {
            x: width / 256.0,
            y: height / 256.0,
        };


        graphics::draw(ctx, sprite, DrawParam::new().dest(self.location)
            .scale(scale_vec))?;

        Ok(())
    }
}



