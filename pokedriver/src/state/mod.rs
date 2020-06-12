pub mod game;
pub mod dialog;

use amethyst::{
    core::math::{Vector2, Vector3},
    ecs::{Entity, World},
};

#[derive(Clone)]
pub enum Trigger {
    DialogEnd
}

pub struct Game {
    trigger: Option<Trigger>,
    pub dead_entities: Vec<Entity>,
    pub camera_trans: Vector3<f32>

}

impl Game {
    pub fn set_trigger(&mut self, trigger: Trigger) {
        self.trigger = Some(trigger);
    }

    pub fn get_trigger(&self) -> Option<Trigger> {
        self.trigger.clone()
    }

    pub fn clear_trigger(&mut self) {
        self.trigger = None;
    }

    pub fn kill_entity(&mut self, entity: Entity) {
        self.dead_entities.push(entity);
    }
}

impl Default for Game {
    fn default() -> Self {
        Game {
            trigger: None,
            dead_entities: Vec::new(),
            camera_trans: Vector3::new(0., 0., 0.)
        }
    }
}



