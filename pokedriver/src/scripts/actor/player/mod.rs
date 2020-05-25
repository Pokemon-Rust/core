pub mod behaviour;
mod walk;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum PlayerBehaviourType {
    None,
    Walk,
    Run,
    Surf,
    Bike
}