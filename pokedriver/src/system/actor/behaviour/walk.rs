use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::Read,
    input::{InputHandler, StringBindings},
    renderer::{SpriteRender, camera::Camera},
};

use crate::system::actor::player::PlayerBehaviour;
use crate::entity::actor::player::Player;
use crate::entity::actor::{ActorDirection, ActorAction};
use crate::utils::resolve::get_fps;


#[derive(Eq, PartialEq)]
enum SpriteTransitionType {
    Walk,
    Turn,
    None,
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum WalkKey {
    Up,
    Down,
    Left,
    Right
}

type Event = Option<WalkKey>;

pub struct Walk {
    counter: usize,
    direction: ActorDirection,
    sprite_transition: SpriteTransitionType,
    action_state: ActorAction,
    bypass_counter: usize,
    key_event: Event,
    active: bool,
    speed: f32,
    capframes: f32
}

impl Walk {
    pub fn new() -> Box<Self> {
        let mut walk = Walk {
            counter: 0,
            direction: ActorDirection::None,
            sprite_transition: SpriteTransitionType::None,
            action_state: ActorAction::Stand,
            bypass_counter: 0,
            key_event: None,
            active: false,
            speed: 3.0,
            capframes: 0.0
        };

        walk.init();
        Box::new(walk)
    }

    pub fn init(&mut self) {
        self.capframes = get_fps() as f32 / self.speed;
    }

    fn cycle_completed(&self) -> bool {
        self.counter == 0
    }

    fn handle(&mut self) {
        self.key_event = None;
    }

    fn get_event(&self, input: &Read<InputHandler<StringBindings>>) -> Event {
        let left = input.action_is_down("left").unwrap_or(false);
        let right = input.action_is_down("right").unwrap_or(false);
        let up = input.action_is_down("up").unwrap_or(false);
        let down = input.action_is_down("down").unwrap_or(false);

        if left {
            Some(WalkKey::Left)
        } else if right {
            Some(WalkKey::Right)
        } else if up {
            Some(WalkKey::Up)
        } else if down {
            Some(WalkKey::Down)
        } else {
            None
        }
    }

    fn set_transition(&mut self, player: &mut Player, direction: ActorDirection) {
        if player.attrs.direction == direction || self.active {
            self.sprite_transition = SpriteTransitionType::Walk;
            player.attrs.direction = direction.clone();
        } else {
            self.sprite_transition = SpriteTransitionType::Turn;
        }

        self.direction = direction.clone();
    }

    fn apply_sprite_transition(&mut self, player: &mut Player, direction: ActorDirection) {
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

            if self.counter < f1_slice as usize {
                player.attrs.action = self.action_state.clone();
            } else if self.counter < f2_slice as usize{
                player.attrs.action = ActorAction::Stand;
            }


            if self.counter == (f2_slice - 1.0) as usize {
                self.handle();
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
            player.attrs.direction = direction;

            if self.counter < f1_slice as usize {
                player.attrs.action = self.action_state.clone();
            } else if self.counter < f2_slice as usize {
                player.attrs.action = ActorAction::Stand;
            }

            if self.counter == (f2_slice - 1.0) as usize {
                self.sprite_transition = SpriteTransitionType::None;
                self.handle();
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

    fn map_to_direction(&mut self, key: Event) -> ActorDirection {
        if key.is_some() {
            match key.unwrap() {
                WalkKey::Up => ActorDirection::North,
                WalkKey::Down => ActorDirection::South,
                WalkKey::Left => ActorDirection::West,
                WalkKey::Right => ActorDirection::East,
            }
        } else {
            ActorDirection::None
        }
    }

    fn evaluate(&mut self, player: &mut Player, input: &Read<InputHandler<StringBindings>>) {
        // wait for any pending key_events and then validate current key_event.
        if self.key_event.is_none() {
            // register a new key_event.
            self.key_event = self.get_event(input);
            if self.key_event.is_some() {
                // set turn-bypass counter.
                let direction = self.map_to_direction(self.key_event);
                if direction == player.attrs.direction {
                    self.active = true;
                    self.bypass_counter = 0;
                }
            }
        } else {
            // when the bypass_counter reaches F (capframes), it is set to 0.
            // indicating that the player's walk momentum is negligible, and any turn operation,
            // will result only in sprite transitions and not viewport transitions.

            self.bypass_counter += 1;
            if self.bypass_counter == self.capframes as usize {
                self.bypass_counter = 0;
                self.active = false;
            }
        }
    }

}

impl PlayerBehaviour for Walk {
    fn run(&mut self, player: &mut Player, _camera: &mut Camera, input: &Read<InputHandler<StringBindings>>) -> bool {
        self.evaluate(player, input);

        if self.key_event.is_some() {
            if self.cycle_completed() {
                let direction = self.map_to_direction(self.key_event.clone());
                self.pre_walk();
                self.set_transition(player, direction);
            }

            self.apply_sprite_transition(player, self.direction.clone());
            self.counter += 1;

            if self.key_event.is_none() || self.counter == get_fps() - 1 {
                self.counter = 0;
            }

            false
        } else {
            true
        }
    }
}