use amethyst::{
    prelude::*,
    ui::{UiText, UiTransform, Anchor},
};
use crate::utils::resolve;

pub fn display_dbg(world: &mut World) {
    let font = resolve::load_font_handle(world);
    let transform = UiTransform::new(
        "text_debug".to_string(),
        Anchor::TopRight,
        Anchor::MiddleRight,
        -0.,
        -30.,
        1.,
        200.,
        30.,
    );

    world.create_entity()
        .with(transform)
        .with(UiText::new(
            font.clone(),
            "Development".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            30.,
        ))
        .build();
}