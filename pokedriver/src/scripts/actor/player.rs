// Actor-script for Player.

use crate::graphics::actor::{Actor, ActorDirection, ActorAction};
use crate::engine::engine::{SharedState};
use ggez::{GameResult, Context};
use std::cell::RefCell;
use ggez::event::KeyCode;
use crate::engine::controller::KeyEvent;
use crate::engine::timer;
use std::time::Duration;

//todo: Implement navigation

fn direct(actor: &mut Actor, direction: ActorDirection) -> bool {
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

fn release_key(actor: &mut Actor, event: KeyEvent) -> bool {
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


pub fn run(actor: &mut Actor, state: &RefCell<SharedState>) -> GameResult<()> {
    let mut curr_state = state.borrow_mut();
    let key_down_event = &curr_state.controller.get_key_down_event();
    let key_up_event = &curr_state.controller.get_key_up_event();

    while timer::check_update_time(&mut actor.time_ctx_group.get(0), 6) {
        if !key_up_event.handled {
            println!("key_up event");

            if key_down_event.handled {
                if release_key(actor, key_up_event.clone()) {
                    curr_state.controller.handle_key_up_event();
                }
            }
        }

        if !key_down_event.handled {
            println!("key_down event");

            let handled = match key_down_event.keycode {
                KeyCode::Up => direct(actor, ActorDirection::North),
                KeyCode::Down => direct(actor, ActorDirection::South),
                KeyCode::Left => direct(actor, ActorDirection::West),
                KeyCode::Right => direct(actor, ActorDirection::East),
                _ => false
            };

            if handled {
                curr_state.controller.handle_key_down_event();
            }
        }



    }


    Ok(())
}