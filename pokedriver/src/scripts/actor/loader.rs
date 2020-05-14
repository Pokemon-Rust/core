use crate::scripts::actor::Script;
use crate::scripts::actor;

pub enum ScriptKey {
    Player
}

pub fn load(key: ScriptKey) -> Script {
    match key {
        ScriptKey::Player => actor::player::run
    }
}