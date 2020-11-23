pub mod player;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ActorDirection {
    North,
    South,
    East,
    West,
    None
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ActorAction {
    Stand,
    Walk1,
    Walk2
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ActorAttrs {
    pub direction: ActorDirection,
    pub action: ActorAction
}

impl ActorAttrs {
    pub fn new() -> Self {
        ActorAttrs {
            direction: ActorDirection::South,
            action: ActorAction::Stand
        }
    }

    pub fn to_sprite_index(&self) -> usize {
        if self.action == ActorAction::Stand ||
            self.action == ActorAction::Walk1 ||
            self.action == ActorAction::Walk2 {
            let index = match self.direction {
                ActorDirection::North => 0,
                ActorDirection::South => 3,
                ActorDirection::East => 6,
                ActorDirection::West => 9,
                _ => 0
            };
            match self.action {
                ActorAction::Stand => index,
                ActorAction::Walk1 => index + 1,
                ActorAction::Walk2 => index + 2
            }
        } else {
            0
        }
    }
}