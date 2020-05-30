mod entity;
mod system;
mod utils;




use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    core::{transform::TransformBundle},
    utils::application_root_dir,
};

use crate::entity::game::GameState;
use crate::system::bundle::GameBundle;

pub fn start() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("../pokedriver/config/display.ron");

    let assets_dir = app_root.join("../pokedriver/assets");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(GameBundle)?
        .with_bundle(
        RenderingBundle::<DefaultBackend>::new()
            .with_plugin(
                RenderToWindow::from_config_path(display_config_path)?
                    .with_clear([0.1, 0.2, 0.3, 1.0]),
            )
            .with_plugin(RenderFlat2D::default()),
    )?;

    let mut game = Application::new(assets_dir, GameState::new(), game_data)?;

    game.run();
    Ok(())
}