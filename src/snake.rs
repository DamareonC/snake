use graphics::types::Rectangle;

use crate::SEGMENT_SIZE;

pub struct SnakeSegment
{
    pub rect: Rectangle,
    pub pos: (f64, f64),
    pub vel: (f64, f64)
}

pub struct Snake
{
    pub segments: Vec<SnakeSegment>,
    pub base_speed: f64,
    pub has_moved: bool
}

impl SnakeSegment
{
    pub fn is_colliding(&self, pos: (f64, f64)) -> bool
    {
        return pos.0 >= self.rect[0] && pos.0 < self.rect[0] + self.rect[2] && pos.1 >= self.rect[1] && pos.1 < self.rect[1] + self.rect[3];
    }
}

impl Snake
{
    pub const COLOR: [f32; 4] = [0.0, 0.75, 0.0, 1.0];

    pub fn add_segment(&mut self)
    {
        let last_segment: &SnakeSegment = self.segments.last().unwrap();
        let new_segment_pos: (f64, f64) = (last_segment.rect[0] - SEGMENT_SIZE * (last_segment.vel.0 / self.base_speed), last_segment.rect[1] - SEGMENT_SIZE * (last_segment.vel.1 / self.base_speed));
        self.segments.push(SnakeSegment {
            rect: [new_segment_pos.0, new_segment_pos.1, SEGMENT_SIZE, SEGMENT_SIZE],
            pos: new_segment_pos,
            vel: last_segment.vel
        });
    }
}