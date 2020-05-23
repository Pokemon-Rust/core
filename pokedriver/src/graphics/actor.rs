use std::cell::RefCell;
use std::collections::HashMap;

use cgmath::{Point2, Vector2};

use ggez::{Context, GameResult, graphics};
use ggez::graphics::DrawParam;

use crate::engine::engine::SharedState;
use crate::scripts::actor;
use crate::utils::resolver;
use crate::graphics::Component;
use crate::graphics::overworld::ViewPort;
use crate::graphics::components::ComponentIdentity;

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum ActorDirection {
    North,
    South,
    East,
    West,
    None
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
    pub fn from(ctx: &mut Context, actor: &String, actor_behaviour_type: &actor::loader::ActorBehaviourType, location: Point2<f32>) -> GameResult<Actor> {
        let mut map = HashMap::new();
        let attribute_batch = resolver::get_actor_attr_batch(ctx, actor)?;

        for attribute in attribute_batch {
            let actor_path = resolver::get_actor_path(ctx, actor, &attribute)?;
            map.insert(attribute.clone(), graphics::Image::new(ctx, actor_path)?);
        }

        let actor = Actor {
            attributes: ActorAttributes {
                direction: ActorDirection::South,
                action: ActorAction::Stand,
            },
            location,
            sprite_map: map.clone(),
            behaviour: actor::loader::load(actor_behaviour_type),
        };

        Ok(actor)
    }
}

impl Component for Actor {
    fn update(&mut self, state: &RefCell<SharedState>) -> GameResult<()> {
        self.behaviour.run(state, &mut self.attributes)?;
        self.behaviour.transform_location(state, &mut self.location);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, view_port: &ViewPort) -> GameResult<()> {
        let sprite = &self.sprite_map[&self.attributes];
        let (width, height) = graphics::drawable_size(ctx);
        let scale_vec = Vector2 {
            x: view_port.scale_x,
            y: view_port.scale_y,
        };


        graphics::draw(ctx, sprite, DrawParam::new().dest(view_port.translate(self.location))
            .scale(scale_vec))?;

        Ok(())
    }

    fn location(&self) -> Point2<f32> {
        self.location
    }

    fn id(&self) -> ComponentIdentity {
        self.behaviour.id()
    }
}



