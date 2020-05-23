use cgmath::Point2;
use ggez::{graphics, Context, GameResult};
use crate::graphics::Component;
use std::cell::RefCell;
use crate::engine::engine::SharedState;
use crate::graphics::components::ComponentIdentity;


type OverWorldLayer = Vec<Box<dyn Component>>;

// The overworld is composed of multiple layers of Components.

type OverWorldLayers = Vec<OverWorldLayer>;

#[derive(Copy, Clone)]
pub struct ViewPort {
    pub origin: Point2<f32>,
    pad_x: f32,
    pad_y: f32,
    pub width: f32,
    pub height: f32,
    pub scale_x: f32,
    pub scale_y: f32
}

impl ViewPort {
    pub fn new() -> ViewPort {
        ViewPort {
            origin: Point2 {
                x: 0.0,
                y: 0.0,
            },
            pad_x: 0.0,
            pad_y: 0.0,
            width: 0.0,
            height: 0.0,
            scale_x: 0.0,
            scale_y: 0.0
        }
    }

    pub fn origin(mut self, pt: Point2<f32>) -> Self {
        self.origin = pt;
        self
    }

    pub fn move_origin(&mut self, dx: f32, dy: f32) {
        self.origin.x += dx * self.scale_x;
        self.origin.y += dy * self.scale_y;
    }

    pub fn init(mut self, ctx: &Context) -> Self {
        let (width, height) = graphics::drawable_size(ctx);
        self.width = width;
        self.height = height;
        self.scale_x = 3.0;
        self.scale_y = 3.0;
        self
    }

    pub fn padding(mut self, pad_x: f32, pad_y: f32) -> Self {
        self.pad_x = pad_x * self.scale_x;
        self.pad_y = pad_y * self.scale_y;
        self
    }

    pub fn translate(self, pt: Point2<f32>) -> Point2<f32> {
        Point2 {
            x: pt.x - self.origin.x,
            y: pt.y - self.origin.y,
        }
    }

    pub fn within_bounds(&self, pt: Point2<f32>) -> bool {
        let trans_pt = self.translate(pt);

        if trans_pt.x > self.width || trans_pt.x + self.pad_x < 0.0 {
            false
        } else if trans_pt.y > self.height || trans_pt.y + self.pad_x < 0.0 {
            false
        } else {
            true
        }
    }
}

pub struct OverWorld {
    layers: OverWorldLayers,
    location: Point2<f32>,
}

impl Component for OverWorld {
    fn update(&mut self, state: &RefCell<SharedState>) -> GameResult<()> {
        for layer_index in 0..self.layers.len() {
            let layer = &mut self.layers[layer_index];
            for renderable in layer {
                renderable.update(state)?;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, view_port: &ViewPort) -> GameResult<()> {
        for layer_index in 0..self.layers.len() {
            let layer = &mut self.layers[layer_index];
            for component in layer.iter_mut() {

                // Draw the component if and only if it is within the viewport.
                if view_port.within_bounds(component.location()) {
                    component.draw(ctx, view_port)?;
                }
            }
        }

        Ok(())
    }

    fn location(&self) -> Point2<f32> {
        self.location
    }

    fn id(&self) -> ComponentIdentity {
        ComponentIdentity::World
    }
}

impl OverWorld {
    pub fn new() -> OverWorld {
        OverWorld {
            layers: Vec::new(),
            location: Point2 {
                x: 0.0,
                y: 0.0,
            },
        }
    }

    // Generates a layer at layer_index if it is non-existent.
    #[inline]
    fn gen_layer(&mut self, layer_index: usize) {
        let mut diff = (layer_index + 1) as i16 - (self.layers.len() as i16);
        while diff > 0 {
            self.layers.push(OverWorldLayer::new());
            diff -= 1;
        }
    }

    // Each element in the overworld has two parameters.
    // 1. A component which handles game logic and rendering of entities.
    // 2. A layer index, which specifies whether an entity should be drawn over/under other components.

    pub fn add(&mut self, elem: Box<dyn Component>, layer_index: usize) {
        self.gen_layer(layer_index);
        self.layers[layer_index].push(elem);
    }
}