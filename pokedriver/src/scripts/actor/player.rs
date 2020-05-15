// Actor-script for Player.

use crate::graphics::actor::{ActorDirection, ActorAction, ActorAttributes, ActorBehaviour};
use crate::engine::engine::{SharedState};
use ggez::{GameResult, Context, graphics};
use ggez::event::KeyCode;
use crate::engine::controller::KeyEvent;
use std::sync::{Mutex, Arc};
use std::borrow::BorrowMut;
use std::collections::HashMap;
use crate::utils::resolver;
use ggez::graphics::DrawParam;
use cgmath::{Vector2, Point2};

//todo: Implement navigation

pub struct PlayerActor {
    pub attributes: ActorAttributes,
    pub action_state: ActorAction,
    sprite_map: HashMap<ActorAttributes, graphics::Image>,
}

impl ActorBehaviour for PlayerActor {
    fn run(&mut self, state: Arc<Mutex<SharedState>>) -> GameResult<()> {
        let mut curr_state = state.lock().unwrap();
        let mut_actor = self;
        let key_down_event = &curr_state.controller.get_key_down_event();
        let key_up_event = &curr_state.controller.get_key_up_event();

        if !key_up_event.handled {
            println!("key_up event");

            if key_down_event.handled {
                if release_key(mut_actor.borrow_mut(), key_up_event.clone()) {
                    curr_state.controller.handle_key_up_event();
                }
            }
        }

        if !key_down_event.handled {
            println!("key_down event");

            let handled = match key_down_event.keycode {
                KeyCode::Up => direct(mut_actor.borrow_mut(), ActorDirection::North),
                KeyCode::Down => direct(mut_actor.borrow_mut(), ActorDirection::South),
                KeyCode::Left => direct(mut_actor.borrow_mut(), ActorDirection::West),
                KeyCode::Right => direct(mut_actor.borrow_mut(), ActorDirection::East),
                _ => false
            };

            if handled {
                curr_state.controller.handle_key_down_event();
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        //todo: implement actor sprite rendering.
        let sprite = &self.sprite_map[&self.attributes];
        let (width, height) = graphics::drawable_size(ctx);
        let scale_vec = Vector2 {
            x: width / 256.0,
            y: height / 256.0,
        };


        graphics::draw(ctx, sprite, DrawParam::new().dest(Point2 {
            x: 100.0,
            y: 100.0
        })
            .scale(scale_vec))?;

        Ok(())
    }
}

impl PlayerActor {
    pub(crate) fn from(ctx: &mut Context, actor: &String, attribute_batch: &Vec<ActorAttributes>) -> GameResult<PlayerActor> {
        let mut map = HashMap::new();

        for attribute in attribute_batch {
            let actor_path = resolver::get_actor_path(ctx, actor, attribute)?;
            map.insert(attribute.clone(), graphics::Image::new(ctx, actor_path)?);
        }

        let actor = PlayerActor {
            attributes: ActorAttributes {
                direction: ActorDirection::South,
                action: ActorAction::Stand,
            },
            sprite_map: map.clone(),
            action_state: ActorAction::Stand,
        };

        Ok(actor)
    }
}


fn direct(actor: &mut PlayerActor, direction: ActorDirection) -> bool {
    if actor.attributes.direction == direction {
        match actor.attributes.action {
            ActorAction::Stand => {
                if actor.action_state == ActorAction::Walk1 {
                    actor.attributes.action = ActorAction::Walk2;
                    actor.action_state = ActorAction::Walk2;
                } else {
                    actor.attributes.action = ActorAction::Walk1;
                    actor.action_state = ActorAction::Walk1;
                }
            }
            ActorAction::Walk1 => {
                actor.action_state = ActorAction::Walk1;
                actor.attributes.action = ActorAction::Stand;
            }
            ActorAction::Walk2 => {
                actor.action_state = ActorAction::Walk2;
                actor.attributes.action = ActorAction::Stand;
            }
        }
    } else {
        actor.attributes.action = ActorAction::Stand;
        actor.attributes.direction = direction;
    }

    true
}

fn release_key(actor: &mut PlayerActor, event: KeyEvent) -> bool {
    match event.keycode {
        KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => {
            actor.attributes.action = ActorAction::Stand;
            true
        }
        _ => false
    }
}

pub fn is_valid_walk(keycode: KeyCode) -> bool {
    match keycode {
        KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => true,
        _ => false
    }
}