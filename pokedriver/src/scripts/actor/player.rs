// Actor-script for Player.

use crate::graphics::actor::{ActorDirection, ActorAction, ActorAttributes};
use crate::engine::engine::{SharedState};
use ggez::GameResult;
use std::cell::RefCell;
use ggez::event::KeyCode;
use crate::engine::controller::KeyEvent;
use crate::engine::timer;
use crate::scripts::actor::ActorBehaviour;
use crate::engine::timer::TimeContextGroup;
use crate::graphics::overworld::ViewPort;
use cgmath::Point2;

//todo: Implement navigation

pub struct PlayerBehaviour {
    action_state: ActorAction,
    time_ctx_group: timer::TimeContextGroup,
}

impl PlayerBehaviour {
    pub fn new() -> PlayerBehaviour {
        PlayerBehaviour {
            action_state: ActorAction::Stand,
            time_ctx_group: TimeContextGroup::new(),
        }
    }

    fn move_view_port(&self, view_port: &mut ViewPort, direction: &ActorDirection) {
        let dx: f32 = 16.0;
        let dy: f32 = 16.0;
        match direction {
            ActorDirection::North => view_port.move_origin(0.0, -dy),
            ActorDirection::South => view_port.move_origin(0.0, dy),
            ActorDirection::East => view_port.move_origin(dx, 0.0),
            ActorDirection::West => view_port.move_origin(-dx, 0.0),
            _ => {}
        }
    }

    fn direct(&mut self, view_port: &mut ViewPort, attr: &mut ActorAttributes, direction: ActorDirection) -> bool {
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

            self.move_view_port(view_port, &direction);
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
    fn run(&mut self, state: &RefCell<SharedState>, attr: &mut ActorAttributes, ) -> GameResult<()> {
        let mut curr_state = state.borrow_mut();
        let key_down_event = &curr_state.controller.get_key_down_event();
        let key_up_event = &curr_state.controller.get_key_up_event();

        while timer::check_update_time(&mut self.time_ctx_group.get(0), 6) {
            if !key_up_event.handled {
                if key_down_event.handled || !self.is_valid_walk(key_down_event.keycode) {
                    if self.release_key(attr, key_up_event.clone()) {
                        curr_state.controller.handle_key_up_event();
                    }
                }
            }

            if !key_down_event.handled {
                let handled = match key_down_event.keycode {
                    KeyCode::Up => self.direct(&mut curr_state.view_port, attr, ActorDirection::North),
                    KeyCode::Down => self.direct(&mut curr_state.view_port, attr, ActorDirection::South),
                    KeyCode::Left => self.direct(&mut curr_state.view_port, attr, ActorDirection::West),
                    KeyCode::Right => self.direct(&mut curr_state.view_port, attr, ActorDirection::East),
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

    fn transform_location(&mut self, state: &RefCell<SharedState>, location: &mut Point2<f32>) {
        let cstate = state.borrow();
        let width = cstate.view_port.width;
        let height = cstate.view_port.height;
        *location = Point2 {
            x: cstate.view_port.origin.x + width / 2.0,
            y: cstate.view_port.origin.y + height / 2.0
        }
    }
}