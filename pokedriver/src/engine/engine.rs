use std::cell::RefCell;
use std::path::PathBuf;

use conf::{Backend, ModuleConf, NumSamples, WindowMode, WindowSetup};
use ggez::{conf, Context, ContextBuilder, event, GameResult, graphics, timer};
use ggez::event::{KeyCode, KeyMods};
use graphics::{DrawParam, Font};
use cgmath::Point2;

use crate::engine::controller::Controller;
use crate::graphics::actor::{ActorAction, ActorAttributes, ActorDirection, ActorBehaviour};
use crate::graphics::sprite::PokemonSprite;
use crate::scripts::actor::loader;
use crate::scripts::actor::loader::ScriptKey;
use crate::utils::resolver::get_fps;
use crate::graphics::sprite::PokemonSpriteType::NormalFront;
use std::sync::{Arc, Mutex};
use std::borrow::BorrowMut;
use crate::scripts::actor::player::PlayerActor;


// The shared state contains fields that are used among different entities for communicating with
// each other.

pub struct SharedState {
    //todo: add relevant fields to SharedState.
    pub controller: Controller
}

impl SharedState {
    pub fn new() -> SharedState {
        SharedState {
            controller: Controller::new()
        }
    }
}

pub struct GameState<'a> {
    dt: std::time::Duration,
    fps_font: Font,
    sprite: PokemonSprite,
    player_actor: Box<ActorBehaviour + 'a>,
    shared_state: Arc<Mutex<SharedState>>,
}

impl<'a> event::EventHandler for GameState<'a> {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.dt = timer::delta(ctx);

        self.player_actor.run(self.shared_state.clone())?;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        while timer::check_update_time(ctx, get_fps() as u32) {
            graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
            let text = graphics::Text::new((format!("{:.0}", timer::fps(ctx)), self.fps_font, 32.0));
            graphics::draw(ctx, &text, DrawParam::default())?;

            self.player_actor.draw(ctx)?;
            self.sprite.draw(ctx, Point2 {
                x: 200.0,
                y: 200.0,
            })?;

            graphics::present(ctx)?;
        }

        timer::yield_now();
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods, _repeat: bool) {
        self.shared_state.lock().unwrap().controller.set_key_down_event(keycode);
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        self.shared_state.lock().unwrap().controller.set_key_up_event(keycode);
    }
}

impl<'a> GameState<'a> {
    pub fn new(ref_mut: RefCell<Context>) -> GameResult<GameState<'a>> {
        let mut ctx = ref_mut.borrow_mut();
        let font = graphics::Font::new(ctx.borrow_mut(), "/fonts/DejaVuSansMono.ttf")?;

        let actor_script = loader::load(ScriptKey::Player, ctx.borrow_mut());
        //todo: create actor attribute batch-maps.
        // testing actor loader.

        let s = GameState {
            dt: std::time::Duration::from_nanos(0),
            fps_font: font,
            sprite: PokemonSprite::from(ctx.borrow_mut(), &"giratina-origin".to_string(), &NormalFront)?,
            player_actor: actor_script,
            shared_state: Arc::new(Mutex::new(SharedState::new())),
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
        let ref_ctx = RefCell::new(ctx);
        let state = &mut GameState::new(RefCell::new(**ref_ctx.borrow()))?;
        event::run(ref_ctx.borrow_mut().borrow_mut(), event_loop, state)
    }
}