use std::cell::RefCell;

use cgmath::Point2;
use ggez::{Context, GameResult, graphics};

use crate::engine::engine::SharedState;
use crate::graphics::Component;
use crate::graphics::components::ComponentIdentity;
use crate::graphics::overworld::ViewPort;
use ggez::graphics::{Font, Text, DrawParam, Mesh, DrawMode, Rect, Color, FillOptions, Align};
use crate::scripts::dialog::DialogBehaviour;
use crate::scripts::dialog::talk_dialog::talk::TalkDialog;

pub struct Dialog {
    behaviour: Box<dyn DialogBehaviour>,
    attrs: DialogAttrs,
}

pub struct DialogAttrs {
    pub text: Vec<String>,
    pub display_text: String,
    pub font: Font,
    pub mesh: Mesh,
    pub mesh_location: Point2<f32>,
    pub text_location: Point2<f32>,
    pub text_bounds: Point2<f32>,
    pub dialog_type: DialogType,
    pub visible: bool,
}

impl DialogAttrs {
    pub fn default(ctx: &mut Context, state: &RefCell<SharedState>) -> GameResult<DialogAttrs> {
        let view_port = state.borrow().view_port;
        let (width, height) = graphics::drawable_size(ctx);
        let location = Point2 {
            x: view_port.origin.x,
            y: view_port.origin.y + height * 0.75,
        };
        Ok(
            DialogAttrs {
                mesh_location: location,
                text_location: location,
                display_text: "".to_string(),
                text_bounds: location.clone(),
                text: vec!["Hello there, and welcome to the world of Pokemon!".to_string(),
                           "Your objective is to screw over your rival.".to_string(),
                           "It shouldn't be hard, he's a RETARD.".to_string()],
                font: graphics::Font::new(ctx, "/fonts/pokemon_fire_red.ttf")?,
                mesh: Mesh::new_rectangle(ctx, DrawMode::Fill(FillOptions::default()),
                                          Rect::new(0.0, 0.0, width, height * 0.25),
                                          Color::from_rgba(0, 0, 0, 153))?,
                dialog_type: DialogType::TalkDialog,
                visible: true,
            }
        )
    }
}

impl Dialog {
    pub fn new(ctx: &mut Context, _text: Vec<String>, _dialog_type: DialogType, _font: Font, state: &RefCell<SharedState>) -> GameResult<Dialog> {
        Ok(Dialog {
            behaviour: Box::new(TalkDialog::new()),
            attrs: DialogAttrs::default(ctx, state)?,
        })
    }
}

impl Component for Dialog {
    fn update(&mut self, state: &RefCell<SharedState>) -> GameResult<()> {
        self.behaviour.run(&mut self.attrs, state)?;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, view_port: &ViewPort) -> GameResult<()> {
        if self.attrs.visible {
            let mut text = Text::new((self.attrs.display_text.clone(), self.attrs.font, 32.0));
            text.set_bounds(self.attrs.text_bounds, Align::Left);

            graphics::draw(ctx, &self.attrs.mesh, DrawParam::new()
                .dest(view_port.translate(self.attrs.mesh_location)))?;

            graphics::draw(ctx, &text, DrawParam::new()
                .dest(view_port.translate(self.attrs.text_location)))?
        }
        Ok(())
    }

    fn location(&self) -> Point2<f32> {
        self.attrs.mesh_location
    }

    fn id(&self) -> ComponentIdentity {
        ComponentIdentity::Dialog(self.attrs.dialog_type)
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum DialogType {
    TalkDialog
}