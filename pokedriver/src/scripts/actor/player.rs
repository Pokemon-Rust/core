// Actor-script for Player.

use crate::graphics::actor::{Actor, ActorDirection, ActorAction, ActorAttributes};
use crate::engine::engine::{SharedState};
use ggez::{GameResult, Context};
use std::cell::RefCell;
use ggez::event::KeyCode;
use crate::engine::controller::KeyEvent;
use crate::engine::timer;
use std::time::Duration;
use crate::scripts::actor::ActorBehaviour;
use crate::engine::timer::TimeContextGroup;

//todo: Implement navigation

pub struct PlayerBehaviour {
    action_state: ActorAction,
    time_ctx_group: timer::TimeContextGroup,
}

impl PlayerBehaviour {
    pub fn new() -> PlayerBehaviour {
        PlayerBehaviour {
            action_state: ActorAction::Stand,
            time_ctx_group: TimeContextGroup::new()
        }
    }

    fn direct(&mut self, attr: &mut ActorAttributes, direction: ActorDirection) -> bool {
        if attr.direction == direction {
            match attr.action {
                ActorAction::Stand => {
                    if self.action_state == ActorAction::Walk1 {
                        attr.action = ActorAction::Walk2;
                        self.action_state = ActorAction::Walk2;
                    } else {
                        attr.action = ActorAction::Walk1;
                        self.action_state = ActorAction::Walk1;
                    }
                }
                ActorAction::Walk1 => {
                    self.action_state = ActorAction::Walk1;
                    attr.action = ActorAction::Stand;
                }
                ActorAction::Walk2 => {
                    self.action_state = ActorAction::Walk2;
                    attr.action = ActorAction::Stand;
                }
            }
        } else {
            attr.action = ActorAction::Stand;
            attr.direction = direction;
        }

        true
    }

    fn release_key(&self, attr: &mut ActorAttributes, event: KeyEvent) -> bool {
        match event.keycode {
            KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => {
                attr.action = ActorAction::Stand;
                true
            }
            _ => false
        }
    }

    fn is_valid_walk(&self, keycode: KeyCode) -> bool {
        match keycode {
            KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => true,
            _ => false
        }
    }

}

impl ActorBehaviour for PlayerBehaviour {
    fn run(&mut self, attr: &mut ActorAttributes, state: &RefCell<SharedState>) -> GameResult<()> {
        let mut curr_state = state.borrow_mut();
        let key_down_event = &curr_state.controller.get_key_down_event();
        let key_up_event = &curr_state.controller.get_key_up_event();

        while timer::check_update_time(&mut self.time_ctx_group.get(0), 12) {
            if !key_up_event.handled {
                println!("key_up event");

                if key_down_event.handled {
                    if self.release_key(attr, key_up_event.clone()) {
                        curr_state.controller.handle_key_up_event();
                    }
                }
            }
        }

        while timer::check_update_time(&mut self.time_ctx_group.get(1), 6) {
            if !key_down_event.handled {
                println!("key_down event");

                let handled = match key_down_event.keycode {
                    KeyCode::Up => self.direct(attr, ActorDirection::North),
                    KeyCode::Down => self.direct(attr, ActorDirection::South),
                    KeyCode::Left => self.direct(attr, ActorDirection::West),
                    KeyCode::Right => self.direct(attr, ActorDirection::East),
                    _ => false
                };

                if handled {
                    curr_state.controller.handle_key_down_event();
                }
            }
        }

        // notify that a frame in this context has been updated.
        self.time_ctx_group.tick_all();

        Ok(())
    }
}