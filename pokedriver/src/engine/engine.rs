use std::cell::RefCell;
use std::path::PathBuf;

use conf::{Backend, ModuleConf, NumSamples, WindowMode, WindowSetup};
use ggez::{conf, Context, ContextBuilder, event, GameResult, graphics, timer};
use graphics::{DrawParam, Font};
use cgmath::Point2;

use crate::engine::controller::Controller;
use crate::graphics::actor::Actor;

use crate::scripts::actor::loader::ActorBehaviourType;
use crate::utils::resolver::get_fps;
use crate::graphics::Component;
use crate::graphics::overworld::{OverWorld, ViewPort};
use crate::graphics::tile::{Tile, TileType};
use crate::engine::input::Input;
use crate::graphics::dialog::{Dialog, DialogType};
use ggez::graphics::Mesh;


// The shared state contains fields that are used among different entities for communicating with
// each other.

pub struct SharedState {
    //todo: add relevant fields to SharedState.
    pub controller: Controller,
    pub view_port: ViewPort,
}

impl SharedState {
    pub fn new() -> SharedState {
        SharedState {
            controller: Controller::new(),
            view_port: ViewPort::new(),
        }
    }
}

pub struct GameState {
    dt: std::time::Duration,
    fps_font: Font,
    input: Input,
    world: Box<dyn Component>,
    shared_state: RefCell<SharedState>,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<GameState> {
        let font = graphics::Font::new(ctx, "/fonts/pokemon_fire_red.ttf")?;
        let shared_state = RefCell::new(SharedState::new());
        // create a static viewport
        let view_port = ViewPort::new().init(ctx)
            .origin(Point2 { x: 0.0, y: 0.0 })
            .padding(16.0, 16.0);

        // Create a vanilla overworld
        let mut world = OverWorld::new();

        world.add(Box::new(Actor::from(ctx, &"brendan".to_string(),
                                       &ActorBehaviourType::Player, Point2 { x: 100.0, y: 100.0 })?), 1);
        world.add(Box::new(Tile::from(ctx, &TileType::GreenPatch, Point2 { x: view_port.origin.x + view_port.width / 2.0 - 24.0, y: view_port.origin.y + view_port.height / 2.0 - 24.0})?), 0);
        world.add(Box::new(Tile::from(ctx, &TileType::GreenPatch, Point2 { x: view_port.origin.x + view_port.width / 2.0 + 24.0, y: view_port.origin.y + view_port.height / 2.0 - 24.0})?), 0);

        world.add(Box::new(Dialog::new(ctx, vec!["Hi there".to_string()], DialogType::TalkDialog, font, &shared_state)?), 2);

        let s = GameState {
            dt: std::time::Duration::from_nanos(0),
            fps_font: font,
            input: Input::new(),
            world: Box::new(world),
            shared_state,
        };

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

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.dt = timer::delta(ctx);
        while timer::check_update_time(ctx, get_fps() as u32) {
            self.input.capture(ctx, &self.shared_state);
            self.world.update(&self.shared_state)?;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        let view_port = self.shared_state.borrow().view_port;
        let vert_line = Mesh::new_line(ctx, &[Point2 {
            x: view_port.width/2.0,
            y: 0.0
        }, Point2 {
            x: view_port.width / 2.0,
            y: view_port.height
        }], 1.0, graphics::WHITE)?;

        let hor_line = Mesh::new_line(ctx, &[Point2 {
            x: 0.0,
            y: view_port.height / 2.0
        }, Point2 {
            x: view_port.width,
            y: view_port.height / 2.0
        }], 1.0, graphics::WHITE)?;



        let text = graphics::Text::new((format!("FPS: {:.0}", timer::fps(ctx)), self.fps_font, 32.0));
        graphics::draw(ctx, &text, DrawParam::default())?;

        self.world.draw(ctx, &self.shared_state.borrow().view_port)?;

        graphics::draw(ctx, &vert_line, DrawParam::default())?;
        graphics::draw(ctx, &hor_line, DrawParam::default())?;

        graphics::present(ctx)?;

        timer::yield_now();
        Ok(())
    }
}

