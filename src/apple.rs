use rand::Rng;
use graphics::types::Rectangle;

use crate::{MAX_WIDTH_RANGE, MAX_HEIGHT_RANGE};

pub struct Apple
{
    pub rect: Rectangle
}

impl Apple
{
    pub const COLOR: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

    pub fn ate(&mut self)
    {
        self.rect[0] = rand::thread_rng().gen_range(0..=MAX_WIDTH_RANGE) as f64;
        self.rect[1] = rand::thread_rng().gen_range(0..=MAX_HEIGHT_RANGE) as f64;
    }
    pub fn is_colliding(&self, pos: (f64, f64)) -> bool
    {
        return pos.0 >= self.rect[0] && pos.0 < self.rect[0] + self.rect[2] && pos.1 >= self.rect[1] && pos.1 < self.rect[1] + self.rect[3];
    }
}