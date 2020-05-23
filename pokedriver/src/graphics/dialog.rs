use std::cell::RefCell;

use cgmath::{Point2, vec1};
use ggez::{Context, GameResult, graphics};

use crate::engine::controller::{Controller, ControllerOwnership, KeyEvent};
use crate::engine::engine::SharedState;
use crate::graphics::Component;
use crate::graphics::components::ComponentIdentity;
use crate::graphics::overworld::ViewPort;
use ggez::graphics::{Font, Text, DrawParam, Mesh, DrawMode, StrokeOptions, Rect, Color, FillOptions};
use crate::scripts::dialog::DialogBehaviour;
use crate::scripts::dialog::talk_dialog::talk::TalkDialog;
use crate::graphics::fsync::FSync;

pub struct Dialog {
    behaviour: Box<dyn DialogBehaviour>,
    attrs: DialogAttrs,
}

pub struct DialogAttrs {
    pub text: Vec<String>,
    pub font: Font,
    pub mesh: Mesh,
    pub location: Point2<f32>,
    pub dialog_type: DialogType,
    pub text_index: usize,
    pub visible: bool,
}

impl DialogAttrs {
    pub fn default(ctx: &mut Context, state: &RefCell<SharedState>) -> GameResult<DialogAttrs> {
        let view_port = state.borrow().view_port;
        let (width, height) = graphics::drawable_size(ctx);
        let location = Point2 {
            x: view_port.origin.x + width / 256.0,
            y: view_port.origin.y + height * 0.75,
        };
        Ok(
            DialogAttrs {
                location,
                text: vec!["Hello there, and welcome to the world of Pokemon!".to_string(), "Your objective is to screw over your rival.".to_string(), "This is line three".to_string()],
                font: graphics::Font::new(ctx, "/fonts/pokemon_fire_red.ttf")?,
                mesh: Mesh::new_rectangle(ctx, DrawMode::Fill(FillOptions::default()),
                                          Rect::new(location.x, location.y, view_port.width - location.x, view_port.height * 0.75),
                                          graphics::WHITE)?,
                dialog_type: DialogType::TalkDialog,
                text_index: 0,
                visible: true,
            }
        )
    }
}

impl Dialog {
    pub fn new(ctx: &mut Context, text: Vec<String>, dialog_type: DialogType, font: Font, state: &RefCell<SharedState>) -> GameResult<Dialog> {
        Ok(Dialog {
            behaviour: Box::new(TalkDialog::new()),
            attrs: DialogAttrs::default(ctx, state)?,
        })
    }
}

impl Component for Dialog {
    fn update(&mut self, state: &RefCell<SharedState>) -> GameResult<()> {
        self.behaviour.run(&mut self.attrs, state)?;
        self.behaviour.transform_location(state, &mut self.attrs.location);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, view_port: &ViewPort) -> GameResult<()> {
        if self.attrs.visible {
            let text = Text::new((self.attrs.text[self.attrs.text_index].clone(), self.attrs.font, 32.0));
            graphics::draw(ctx, &self.attrs.mesh, DrawParam::new()
                .dest(view_port.translate(self.attrs.location)))?;
            graphics::draw(ctx, &text, DrawParam::new()
                .dest(view_port.translate(self.attrs.location)))?
        }
        Ok(())
    }

    fn location(&self) -> Point2<f32> {
        self.attrs.location
    }

    fn id(&self) -> ComponentIdentity {
        ComponentIdentity::Dialog(self.attrs.dialog_type)
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum DialogType {
    TalkDialog
}