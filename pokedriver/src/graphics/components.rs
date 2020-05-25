use crate::scripts::actor::player::PlayerBehaviourType;
use crate::graphics::tile::TileType;
use crate::graphics::dialog::DialogType;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum ComponentIdentity {
    World,
    Player(PlayerBehaviourType),
    Tile(TileType),
    Dialog(DialogType)
}