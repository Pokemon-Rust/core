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
use crate::graphics::fsync::FSync;
use cgmath::Point2;
use std::time::Duration;
use crate::utils::resolver;

//todo: Implement navigation

#[derive(Eq, PartialEq)]
enum SpriteTransitionType {
    Walk,
    Turn,
    None
}

pub struct PlayerBehaviour {
    action_state: ActorAction,
    fsync: FSync,
    transition: f32,
    direction: ActorDirection,
    speed: f32,
    capframes: f32,
    handled: bool,
    sprite_transition: SpriteTransitionType
}



impl PlayerBehaviour {
    pub fn new() -> PlayerBehaviour {
        PlayerBehaviour {
            action_state: ActorAction::Stand,
            fsync: FSync::new().set_frames(resolver::get_fps()),
            transition: 0.0,
            direction: ActorDirection::None,
            speed: 2.0,
            capframes: 0.0,
            handled: false,
            sprite_transition: SpriteTransitionType::None
        }
    }

    pub fn speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self.capframes = resolver::get_fps() as f32 / speed;
        self
    }

    fn apply_viewport_transition(&mut self, view_port: &mut ViewPort) {
        let mut slice = self.transition / self.capframes;
        if self.transition > 0.0 {
            if self.transition < slice {
                slice = self.transition;
            }
            match self.direction {
                ActorDirection::North => view_port.move_origin(0.0, -slice),
                ActorDirection::South => view_port.move_origin(0.0, slice),
                ActorDirection::East => view_port.move_origin(slice, 0.0),
                ActorDirection::West => view_port.move_origin(-slice, 0.0),
                _ => {}
            };
            self.transition -= slice;
        }
    }

    fn apply_sprite_transition(&mut self, attr: &mut ActorAttributes, direction: ActorDirection) {
        let capframes = self.capframes;
        // Turns need to be faster than walks.
        if self.sprite_transition == SpriteTransitionType::Walk {
            // Sprite transition for walks:- (Stand) -> Walk_i -> Stand
            // Frame Definition:
            // 0..F/2 :- Walk_i
            // F/2..F :- Stand

            let slice = capframes / 2.0;
            let f1_slice = slice;
            let f2_slice = 2.0 * slice;

            if self.fsync.get_frame_f32() < f1_slice {
                attr.action = self.action_state.clone();
            } else if self.fsync.get_frame_f32() < f2_slice {
                attr.action = ActorAction::Stand;
            }


            if self.fsync.get_event_frame() == (f2_slice - 1.0) as u16 {
                self.sprite_transition = SpriteTransitionType::None;
                self.handled = true;
            }

        } else if self.sprite_transition == SpriteTransitionType::Turn {
            // Sprite transition for turn:- (Stand) -> Walk_i -> Stand
            // Frame Definition:
            // 0..F/6 :- Walk_i
            // F/6..F/3 :- Stand
            // handle keydown event.

            let slice = capframes / 6.0;
            let f1_slice = slice;
            let f2_slice = 2.0 * slice;

            // Change direction.
            attr.direction = direction;

            if self.fsync.get_frame_f32() < f1_slice {
                attr.action = self.action_state.clone();
            } else if self.fsync.get_frame_f32() < f2_slice {
                attr.action = ActorAction::Stand;
            }

            if self.fsync.get_event_frame() == (f2_slice - 1.0) as u16 {
                self.sprite_transition = SpriteTransitionType::None;
                self.handled = true;
            }
        }
    }

    fn pre_walk(&mut self) {
        if self.action_state == ActorAction::Walk1 {
            self.action_state = ActorAction::Walk2;
        } else {
            self.action_state = ActorAction::Walk1;
        }
    }

    #[inline]
    fn try_handle(&mut self) {
        if self.fsync.get_event_frame() == resolver::get_fps() - 1 {
            self.handled = true;
        }
    }

    #[inline]
    fn map_to_direction(&mut self, keyevent: KeyEvent) -> ActorDirection {
        match keyevent.keycode {
            KeyCode::Up => ActorDirection::North,
            KeyCode::Down => ActorDirection::South,
            KeyCode::Left => ActorDirection::West,
            KeyCode::Right => ActorDirection::East,
            _ => ActorDirection::None
        }
    }

    fn set_transition(&mut self, view_port: &mut ViewPort, attr: &mut ActorAttributes, direction: ActorDirection) {
        // if player is moving in the same direction, we need a viewport transition.
        if attr.direction == direction {
            let dx: f32 = 16.0 * view_port.width / 256.0;
            let dy: f32 = 16.0 * view_port.height / 256.0;
            self.transition = match direction {
                ActorDirection::North | ActorDirection::South => dy,
                ActorDirection::East | ActorDirection::West => dx,
                _ => 0.0
            };

            self.sprite_transition = SpriteTransitionType::Walk;
        } else {
            self.transition = 0.0;
            self.sprite_transition = SpriteTransitionType::Turn;
        }

        self.direction = direction.clone();
    }

    fn is_valid_walk(&self, key_event: KeyEvent) -> bool {
        match key_event.keycode {
            KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => true,
            _ => false
        }
    }
}

impl ActorBehaviour for PlayerBehaviour {
    fn run(&mut self, state: &RefCell<SharedState>, attr: &mut ActorAttributes) -> GameResult<()> {
        let mut curr_state = state.borrow_mut();
        let key_down_event = &curr_state.controller.get_key_down_event();

        if !key_down_event.handled {
            if self.is_valid_walk(key_down_event.clone()) {
                if self.fsync.get_event_frame() == 0 {
                    let direction = self.map_to_direction(key_down_event.clone());
                    self.pre_walk();
                    self.set_transition(&mut curr_state.view_port, attr, direction.clone());
                }

                self.apply_viewport_transition(&mut curr_state.view_port);
                self.apply_sprite_transition(attr, self.direction.clone());

                self.try_handle();

                self.fsync.update();

                if self.handled {
                    self.handled = false;
                    curr_state.controller.handle_key_down_event();
                    self.fsync.reset_frames();
                }
            }
        }

        Ok(())
    }

    fn transform_location(&mut self, state: &RefCell<SharedState>, location: &mut Point2<f32>) {
        let mut cstate = state.borrow_mut();
        let width = cstate.view_port.width;
        let height = cstate.view_port.height;


        *location = Point2 {
            x: cstate.view_port.origin.x + width / 2.0,
            y: cstate.view_port.origin.y + height / 2.0,
        }
    }
}