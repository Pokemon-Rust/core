use std::path::PathBuf;

use conf::{Backend, ModuleConf, NumSamples, WindowMode, WindowSetup};
use ggez::{conf, Context, ContextBuilder, event, GameResult, graphics, timer};
use graphics::{DrawParam, Font};
use cgmath::Point2;
use std::cell::RefCell;

use crate::graphics::sprite::{PokemonSprite, PokemonSpriteType};
use crate::utils::resolver;
use crate::graphics::actor::{Actor, ActorDirection, ActorAction, ActorAttributes};
use crate::scripts::actor::loader;
use crate::scripts::actor::loader::ScriptKey;


// The shared state contains fields that are used among different entities for communicating with
// each other.

pub struct SharedState {
    //todo: add relevant fields to SharedState.
    pub test: u32
}

impl SharedState {
    pub fn new() -> SharedState {
        SharedState {
            test: 10
        }
    }
}

pub struct GameState {
    dt: std::time::Duration,
    fps_font: Font,
    sprite: PokemonSprite,
    actor: Actor,
    shared_state: RefCell<SharedState>,
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.dt = timer::delta(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let desired_fps = resolver::get_fps();
        while timer::check_update_time(ctx, desired_fps as u32) {
            graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
            let text = graphics::Text::new((format!("{:.0}", timer::fps(ctx)), self.fps_font, 32.0));
            graphics::draw(ctx, &text, DrawParam::default())?;

            self.sprite.draw(ctx, Point2::new(100.0, 100.0))?;

            self.actor.draw(ctx, &self.shared_state)?;


            graphics::present(ctx)?;
        }

        timer::yield_now();
        Ok(())
    }
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<GameState> {
        let font = graphics::Font::new(ctx, "/fonts/DejaVuSansMono.ttf")?;

        let actor_script = loader::load(ScriptKey::Player);
        //todo: create actor attribute batch-maps.
        // testing actor loader.

        let attribute_batch = vec![
            ActorAttributes {
                direction: ActorDirection::South,
                action: ActorAction::Stand,
            },
        ];

        let s = GameState {
            dt: std::time::Duration::from_nanos(0),
            fps_font: font,
            sprite: PokemonSprite::from(ctx, &"pikachu".to_string(), &PokemonSpriteType::NormalFront)?,
            actor: Actor::from(ctx, &"brendan".to_string(),
                               &attribute_batch, &actor_script)?,
            shared_state: RefCell::new(SharedState::new()),
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
                srgb: true,
            },
            backend: Backend::default(),
            modules: ModuleConf::default(),
        };
        let cb = ContextBuilder::new("Pokemon Rust", "SphericalKat & Supercmmetry")
            .conf(conf)
            .add_resource_path(res_path);
        let (ctx, event_loop) = &mut cb.build()?;

        let state = &mut GameState::new(ctx)?;
        event::run(ctx, event_loop, state)
    }
}