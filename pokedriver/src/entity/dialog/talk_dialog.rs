use amethyst::{
    prelude::*,
    core::{
        transform::Transform,
        math::Vector3,
    },
    ecs::{Component, DenseVecStorage},
    renderer::{SpriteSheet, SpriteRender},
    assets::Handle,
};

pub struct TalkDialog {
    text: Vec<String>,
    index: usize,
    char_index: usize
}

impl TalkDialog {
    pub fn new() -> Self {
        TalkDialog {
            text: Vec::new(),
            index: 0,
            char_index: 0
        }
    }
}


impl Component for TalkDialog {
    type Storage = DenseVecStorage<Self>;
}
