use core::fmt;

pub mod tile;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TileClass {
    GreenPatch
}

impl fmt::Display for TileClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TileAttrs {
    class: TileClass,
    state: usize
}

impl TileAttrs {
    pub fn to_sprite_index(&self) -> usize {
        if let Some(class) = self.class {
            let class_index = match class {
                TileClass::GreenPatch => 0
            };
            class_index + self.state
        } else {
            0
        }
    }
}