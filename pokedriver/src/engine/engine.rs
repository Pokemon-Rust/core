use std::cell::RefCell;
use std::path::PathBuf;

use conf::{Backend, ModuleConf, NumSamples, WindowMode, WindowSetup};
use ggez::{conf, Context, ContextBuilder, event, GameResult, graphics, timer};
use ggez::event::{KeyCode, KeyMods};
use graphics::{DrawParam, Font};
use cgmath::Point2;

use crate::engine::controller::Controller;
use crate::graphics::actor::{Actor, ActorAction, ActorAttributes, ActorDirection};
use crate::scripts::actor::loader::ActorBehaviourType;
use crate::utils::resolver::get_fps;
use crate::graphics::Renderable;
use crate::graphics::overworld::{OverWorld, ViewPort};


// The shared state contains fields that are used among different entities for communicating with
// each other.

pub struct SharedState {
    //todo: add relevant fields to SharedState.
    pub controller: Controller,
    pub view_port: ViewPort
}

impl SharedState {
    pub fn new() -> SharedState {
        SharedState {
            controller: Controller::new(),
            view_port: ViewPort::new()
        }
    }
}

pub struct GameState {
    dt: std::time::Duration,
    fps_font: Font,
    world: Box<dyn Renderable>,
    shared_state: RefCell<SharedState>,
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.dt = timer::delta(ctx);

        self.world.update(&self.shared_state)?;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        while timer::check_update_time(ctx, get_fps() as u32) {
            graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
            let text = graphics::Text::new((format!("FPS: {:.0}", timer::fps(ctx)), self.fps_font, 32.0));
            graphics::draw(ctx, &text, DrawParam::default())?;

            // draw overworld at (0,0)
            self.world.draw(ctx, &self.shared_state.borrow().view_port)?;

            graphics::present(ctx)?;
        }

        timer::yield_now();
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods, _repeat: bool) {
        self.shared_state.borrow_mut().controller.set_key_down_event(keycode);
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        self.shared_state.borrow_mut().controller.set_key_up_event(keycode);
    }
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<GameState> {
        let font = graphics::Font::new(ctx, "/fonts/DejaVuSansMono.ttf")?;

        //todo: create actor attribute batch-maps.
        // testing actor loader.

        let attribute_batch = vec![
            ActorAttributes {
                direction: ActorDirection::South,
                action: ActorAction::Stand,
            }, ActorAttributes {
                direction: ActorDirection::North,
                action: ActorAction::Stand,
            }, ActorAttributes {
                direction: ActorDirection::East,
                action: ActorAction::Stand,
            }, ActorAttributes {
                direction: ActorDirection::West,
                action: ActorAction::Stand,
            }, ActorAttributes {
                direction: ActorDirection::South,
                action: ActorAction::Walk1,
            }, ActorAttributes {
                direction: ActorDirection::North,
                action: ActorAction::Walk1,
            }, ActorAttributes {
                direction: ActorDirection::East,
                action: ActorAction::Walk1,
            }, ActorAttributes {
                direction: ActorDirection::West,
                action: ActorAction::Walk1,
            },
            ActorAttributes {
                direction: ActorDirection::South,
                action: ActorAction::Walk2,
            }, ActorAttributes {
                direction: ActorDirection::North,
                action: ActorAction::Walk2,
            }, ActorAttributes {
                direction: ActorDirection::East,
                action: ActorAction::Walk2,
            }, ActorAttributes {
                direction: ActorDirection::West,
                action: ActorAction::Walk2,
            },
        ];

        // Create a vanilla overworld

        let mut world = OverWorld::new();

        world.add(Box::new(Actor::from(ctx, &"brendan".to_string(),
                                       &attribute_batch, &ActorBehaviourType::Player)?), 0);


        let s = GameState {
            dt: std::time::Duration::from_nanos(0),
            fps_font: font,
            world: Box::new(world),
            shared_state: RefCell::new(SharedState::new()),
        };

        // create a static viewport
        let view_port = ViewPort::new().init(ctx)
            .origin(Point2 {x: 0.0, y: 0.0})
            .padding(2.0);

        s.shared_state.borrow_mut().view_port = view_port;

        Ok(s)
    }

    pub fn start() -> GameResult<()> {
        let res_path = PathBuf::from("../pokedriver/resources");
        let conf = conf::Conf {
            window_mode: WindowMode::default(),
            window_setup: WindowSetup {
                title: "Pokemon Rust".to_string(),
                samples: NumSamples::Zero,
                vsync: false,
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