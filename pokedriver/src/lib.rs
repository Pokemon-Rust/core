mod entity;
mod state;
mod system;
mod utils;



use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    input::{InputBundle, StringBindings},
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

use crate::state::game::GameState;
use crate::system::bundle::GameBundle;
use std::time::Duration;
use crate::utils::resolve;

pub fn start() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("../pokedriver/config/display.ron");

    let assets_dir = app_root.join("../pokedriver/assets");
    let input_binding_config = app_root.join("../pokedriver/config/input.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<StringBindings>::new()
            .with_bindings_from_file(input_binding_config)?)?
        .with_bundle(GameBundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(
        RenderingBundle::<DefaultBackend>::new()
            .with_plugin(
                RenderToWindow::from_config_path(display_config_path)?
                    .with_clear([0.1, 0.2, 0.3, 1.0]),
            )
            .with_plugin(RenderFlat2D::default())
            .with_plugin(RenderUi::default()),
    )?;

    let mut game = Application::build(assets_dir, GameState::new())?
        .with_frame_limit(
        FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(1)),
        resolve::get_fps() as u32)
        .build(game_data)?;

    game.run();
    Ok(())
}