pub mod talk_dialog;

pub enum DialogSpritetype {
    Bottom,
    OptionBox
}

impl DialogSpritetype {
    pub fn to_sprite_index(&self) -> usize {
        match self {
            DialogSpritetype::Bottom => 0,
            DialogSpritetype::OptionBox => 1
        }
    }
}




