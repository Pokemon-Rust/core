pub mod game;
pub mod dialog;

use amethyst::core::math::Vector2;

#[derive(Clone)]
pub enum Trigger {
    DialogEnd
}

pub struct Game {
    trigger: Option<Trigger>,
}

impl Game {
    pub fn set_trigger(&mut self, trigger: Trigger) {
        self.trigger =  Some(trigger);
    }

    pub fn get_trigger(&self) -> Option<Trigger> {
        self.trigger.clone()
    }

    pub fn clear_trigger(&mut self) {
        self.trigger = None;
    }
}

impl Default for Game {
    fn default() -> Self {
        Game {
            trigger: None,
        }
    }
}



