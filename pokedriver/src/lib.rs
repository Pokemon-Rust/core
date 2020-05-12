use std::path::PathBuf;

use conf::{Backend, ModuleConf, NumSamples, WindowMode, WindowSetup};
use ggez::{conf, Context, ContextBuilder, event, GameResult, graphics, timer};
use graphics::{DrawParam, Font};

pub struct GameState {
    dt: std::time::Duration,
    fps_font: Font,
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.dt = timer::delta(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        let text = graphics::Text::new((format!("{:.0}", timer::fps(ctx)), self.fps_font, 32.0));
        graphics::draw(ctx, &text, DrawParam::default())?;

        graphics::present(ctx)?;
        Ok(())
    }
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<GameState> {
        let font = graphics::Font::new(ctx, "/fonts/DejaVuSansMono.ttf")?;
        let s = GameState {
            dt: std::time::Duration::from_nanos(0),
            fps_font: font,
        };
        Ok(s)
    }

    pub fn start() -> GameResult<()> {
        let res_path = PathBuf::from("../pokedriver/resources");
        let conf = conf::Conf {
            window_mode: WindowMode::default(),
            window_setup: WindowSetup {
                title: "Pokemon Rust".to_string(),
                samples: NumSamples::Zero,
                vsync: true,
                icon: "".to_string(),
                srgb: true
            },
            backend: Backend::default(),
            modules: ModuleConf::default()
        };
        let cb = ContextBuilder::new("Pokemon Rust", "SphericalKat & Supercmmetry")
            .conf(conf)
            .add_resource_path(res_path);
        let (ctx, event_loop) = &mut cb.build()?;

        let state = &mut GameState::new(ctx)?;
        event::run(ctx, event_loop, state)
    }
}