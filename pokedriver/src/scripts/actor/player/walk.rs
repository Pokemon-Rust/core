use crate::graphics::actor::{ActorAction, ActorDirection, ActorAttributes};
use crate::graphics::fsync::FSync;
use crate::utils::resolver;
use ggez::input::keyboard::KeyCode;
use crate::engine::controller::{Controller, KeyEvent, ControllerOwnership};
use crate::scripts::actor::ActorBehaviour;
use std::cell::RefCell;
use crate::engine::engine::SharedState;
use ggez::GameResult;
use cgmath::Point2;
use crate::graphics::components::ComponentIdentity;
use crate::scripts::actor::player::PlayerBehaviourType;

#[derive(Eq, PartialEq)]
enum SpriteTransitionType {
    Walk,
    Turn,
    None,
}

#[derive(ControllerOwnership)]
pub struct WalkBehaviour {
    action_state: ActorAction,
    fsync: FSync,
    transition: f32,
    transition_slice: f32,
    direction: ActorDirection,
    speed: f32,
    capframes: f32,
    key_event: KeyEvent,
    sprite_transition: SpriteTransitionType,
    is_walking: bool,
    bypass_counter: usize,
    dbg_count: usize
}

impl WalkBehaviour {
    pub fn new() -> WalkBehaviour {
        WalkBehaviour {
            action_state: ActorAction::Stand,
            fsync: FSync::new().set_frames(resolver::get_fps()),
            transition: 0.0,
            transition_slice: 0.0,
            direction: ActorDirection::None,
            speed: 1.0,
            capframes: 0.0,
            key_event: KeyEvent::new(),
            sprite_transition: SpriteTransitionType::None,
            is_walking: false,
            bypass_counter: 0,
            dbg_count: 0
        }
    }

    pub fn speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self.capframes = resolver::get_fps() as f32 / speed;
        self
    }

    fn apply_viewport_transition(&mut self, state: &RefCell<SharedState>) {
        let view_port = &mut state.borrow_mut().view_port;

        let mut slice = self.transition_slice;
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
            self.dbg_count += 1;
        } else {
            self.transition = 0.0;
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
            let f2_slice = capframes;

            if self.fsync.get_event_frame() < f1_slice as u16 {
                attr.action = self.action_state.clone();
            } else if self.fsync.get_event_frame() < f2_slice as u16 {
                attr.action = ActorAction::Stand;
            }


            if self.fsync.get_event_frame() == (f2_slice - 1.0) as u16 {
                //self.sprite_transition = SpriteTransitionType::None;
                self.key_event.handled = true;
            }
        } else if self.sprite_transition == SpriteTransitionType::Turn {
            // Sprite transition for turn:- (Stand) -> Walk_i -> Stand
            // Frame Definition:
            // 0..F/3 :- Walk_i
            // F/3..2F/3 :- Stand
            // handle keydown event.

            let slice = capframes / 3.0;
            let f1_slice = slice;
            let f2_slice = 2.0 * slice;

            // Change direction.
            attr.direction = direction;

            if self.fsync.get_event_frame() < f1_slice as u16 {
                attr.action = self.action_state.clone();
            } else if self.fsync.get_event_frame() < f2_slice as u16 {
                attr.action = ActorAction::Stand;
            }

            if self.fsync.get_event_frame() == (f2_slice - 1.0) as u16 {
                self.sprite_transition = SpriteTransitionType::None;
                self.key_event.handled = true;
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
            self.key_event.handled = true;
        }
    }

    #[inline]
    fn map_to_direction(&mut self, keycode: KeyCode) -> ActorDirection {
        match keycode {
            KeyCode::Up => ActorDirection::North,
            KeyCode::Down => ActorDirection::South,
            KeyCode::Left => ActorDirection::West,
            KeyCode::Right => ActorDirection::East,
            _ => ActorDirection::None
        }
    }

    fn set_transition(&mut self, attr: &mut ActorAttributes, direction: ActorDirection, state: &RefCell<SharedState>) {
        // if player is moving in the same direction, we need a viewport transition.
        let view_port = state.borrow().view_port;
        if attr.direction == direction || self.is_walking {
            let dx: f32 = 16.0;
            let dy: f32 = 16.0;
            self.transition = match direction {
                ActorDirection::North | ActorDirection::South => dy,
                ActorDirection::East | ActorDirection::West => dx,
                _ => 0.0
            };

            self.transition_slice = self.transition / self.capframes;

            self.sprite_transition = SpriteTransitionType::Walk;
            attr.direction = direction.clone();
        } else {
            self.transition = 0.0;
            self.transition_slice = 0.0;
            self.sprite_transition = SpriteTransitionType::Turn;
        }

        self.direction = direction.clone();
    }

    fn set_walk_key(&mut self, controller: &Controller) {
        let mut keycode = self.key_event.key;

        if !controller.is_keydown(keycode) {
            if controller.is_keydown(KeyCode::Up) {
                keycode = KeyCode::Up;
            } else if controller.is_keydown(KeyCode::Down) {
                keycode = KeyCode::Down;
            } else if controller.is_keydown(KeyCode::Left) {
                keycode = KeyCode::Left;
            } else if controller.is_keydown(KeyCode::Right) {
                keycode = KeyCode::Right;
            }
        }

        self.key_event.key = keycode;
    }

    fn handle_keydown(&self, controller: &mut Controller) {
        controller.handle_keydown(KeyCode::Up);
        controller.handle_keydown(KeyCode::Down);
        controller.handle_keydown(KeyCode::Left);
        controller.handle_keydown(KeyCode::Right);
    }

    fn is_valid_walk(&self, controller: &Controller) -> bool {
        controller.is_keydown(KeyCode::Up) ||
            controller.is_keydown(KeyCode::Down) ||
            controller.is_keydown(KeyCode::Left) ||
            controller.is_keydown(KeyCode::Right)
    }

    fn evaluate(&mut self, state: &RefCell<SharedState>, attr: &ActorAttributes) {
        let controller = &mut state.borrow_mut().controller;
        // wait for any pending key_events and then validate current key_event.
        if self.key_event.handled && self.is_valid_walk(controller) {
            // register a new key_event.
            self.key_event.handled = false;
            self.set_walk_key(controller);

            // set turn-bypass counter.
            let direction = self.map_to_direction(self.key_event.key);
            if direction == attr.direction {
                self.is_walking = true;
                self.bypass_counter = 0;
            }
        } else {
            // when the bypass_counter reaches F (capframes), it is set to 0.
            // indicating that the player's walk momentum is negligible, and any turn operation,
            // will result only in sprite transitions and not viewport transitions.

            self.bypass_counter += 1;
            if self.bypass_counter == self.capframes as usize {
                self.bypass_counter = 0;
                self.is_walking = false;
            }
        }

        // finally, we consume any key-events relevant to this behaviour, to prevent unwanted propagation.
        self.handle_keydown(controller);
    }
}

impl ActorBehaviour for WalkBehaviour {
    fn run(&mut self, state: &RefCell<SharedState>, attr: &mut ActorAttributes) -> GameResult<()> {

        // Try to own the controller. If it fails, skip.
        if self.own(state) {
            self.evaluate(state, attr);

            if !self.key_event.handled {
                let pressed_key = self.key_event.key;

                if self.fsync.cycle_completed() {
                    let direction = self.map_to_direction(pressed_key);
                    self.pre_walk();
                    self.set_transition(attr, direction.clone(), state);
                }

                self.apply_viewport_transition(state);
                self.apply_sprite_transition(attr, self.direction.clone());
                self.try_handle();
                self.fsync.update();

                if self.key_event.handled {
                    self.fsync.reset_frames();
                }
            }

            // Unlock the controller.
            self.disown(state);
        }

        Ok(())
    }

    fn transform_location(&mut self, state: &RefCell<SharedState>, location: &mut Point2<f32>) {
        let cstate = state.borrow_mut();
        let width = cstate.view_port.width;
        let height = cstate.view_port.height;


        *location = Point2 {
            x: cstate.view_port.origin.x + width / 2.0 - 8.0 * cstate.view_port.scale_x,
            y: cstate.view_port.origin.y + height / 2.0 - 18.0 * cstate.view_port.scale_y,
        }
    }

    fn id(&self) -> ComponentIdentity {
        ComponentIdentity::Player(PlayerBehaviourType::Walk)
    }
}


