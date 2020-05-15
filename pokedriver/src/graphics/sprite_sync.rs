use crate::utils::resolver;

pub struct SpriteSync {
    event_loop_frame_id: u16,
    frame_id: f32,
    n_frames: u16,
}

impl SpriteSync {
    pub fn new() -> SpriteSync {
        SpriteSync {
            event_loop_frame_id: 0,
            frame_id: 0.0,
            n_frames: 0,
        }
    }

    pub fn update(&mut self) {
        let desired_fps = resolver::get_fps();

        if self.event_loop_frame_id == desired_fps - 1 {
            self.event_loop_frame_id = 0;
        } else {
            self.event_loop_frame_id += 1;
        }

        if self.frame_id.ceil() as u16 >= self.n_frames - 1 {
            self.frame_id = 0.0;
        } else {
            if self.n_frames < desired_fps {
                self.frame_id = ((self.event_loop_frame_id as f32) * (self.n_frames as f32)) / (desired_fps as f32);
            } else {
                self.frame_id += 1.0;
            }
        }
    }

    pub fn set_frames(mut self, frames: u16) -> Self {
        self.n_frames = frames;
        self
    }

    pub fn get_frame(&self) -> usize {
        self.frame_id.floor() as usize
    }

}