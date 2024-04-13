use piston::{Button, Key, RenderArgs, UpdateArgs};
use opengl_graphics::GlGraphics;

use crate::{snake::Snake, apple::Apple, WINDOW_WIDTH, WINDOW_HEIGHT, SEGMENT_SIZE};

pub struct Game
{
    pub(crate) gl: GlGraphics,
    pub(crate) score: i32,
    pub(crate) game_over: bool
}

impl Game
{
    pub(crate) fn render(&mut self, snake: &Snake, apple: &Apple, args: RenderArgs)
    {
        use graphics::*;

        self.gl.draw(args.viewport(), |c: Context, gl: &mut GlGraphics|
        {
            clear([0.0, 0.0, 0.0, 1.0], gl);
            for segment in snake.segments.iter()
            {
                rectangle(Snake::COLOR, segment.rect, c.transform.trans(0.0, 0.0), gl);
            }
            rectangle(Apple::COLOR, apple.rect, c.transform.trans(0.0, 0.0), gl);
        });
    }
    pub(crate) fn update(&mut self, snake: &mut Snake, apple: &mut Apple, _args: UpdateArgs)
    {
        let offscreen: bool = snake.segments[0].rect[0] < 0.0 || snake.segments[0].rect[0] + snake.segments[0].rect[2] >= WINDOW_WIDTH + SEGMENT_SIZE || snake.segments[0].rect[1] < 0.0 || snake.segments[0].rect[1] + snake.segments[0].rect[3] >= WINDOW_HEIGHT + SEGMENT_SIZE;

        let head_top_left: (f64, f64) = (snake.segments.get(0).unwrap().rect[0], snake.segments.get(0).unwrap().rect[1]);
        let head_top_right: (f64, f64) = (snake.segments.get(0).unwrap().rect[0] + snake.segments.get(0).unwrap().rect[2], snake.segments.get(0).unwrap().rect[1]);
        let head_bottom_left: (f64, f64) = (snake.segments.get(0).unwrap().rect[0], snake.segments.get(0).unwrap().rect[1] + snake.segments.get(0).unwrap().rect[3]);
        let head_bottom_right: (f64, f64) = (snake.segments.get(0).unwrap().rect[0] + snake.segments.get(0).unwrap().rect[2], snake.segments.get(0).unwrap().rect[1] + snake.segments.get(0).unwrap().rect[3]);
        
        snake.segments[0].rect[0] += snake.segments[0].vel.0;
        snake.segments[0].rect[1] += snake.segments[0].vel.1;
        
        if snake.segments.len() > 1
        {
            for i in 1..snake.segments.len()
            {
                snake.segments[i].rect[0] += snake.segments[i].vel.0;
                snake.segments[i].rect[1] += snake.segments[i].vel.1;

                if snake.segments[i].rect[0] == snake.segments[i - 1].pos.0 && snake.segments[i].rect[1] == snake.segments[i - 1].pos.1 && ((snake.segments[i].rect[0] - snake.segments[i].pos.0).abs() >= SEGMENT_SIZE || (snake.segments[i].rect[1] - snake.segments[i].pos.1).abs() >= SEGMENT_SIZE)
                {
                    snake.segments[i].pos = (snake.segments[i].rect[0], snake.segments[i].rect[1]);
                    snake.segments[i].vel = snake.segments[i - 1].vel;
                }
                
                if i > 2 //Segments that are neither the head nor the 2 segments behind the head
                {
                    if offscreen || snake.segments[i].is_colliding(head_top_left) || snake.segments[i].is_colliding(head_top_right) || snake.segments[i].is_colliding(head_bottom_left) || snake.segments[i].is_colliding(head_bottom_right)
                    {
                        self.game_over = true;
                    }
                }
            }
        }

        if apple.is_colliding(head_top_left) || apple.is_colliding(head_top_right) || apple.is_colliding(head_bottom_left) || apple.is_colliding(head_bottom_right)
        {
            self.score += 1;
            apple.ate();
            snake.add_segment();
        }

        if offscreen
        {
            self.game_over = true;
        }
    }
    pub(crate) fn key_pressed(&self, snake: &mut Snake, args: Button)
    {
        if let Button::Keyboard(key) = args
        {
            match key
            {
                Key::Up | Key::W =>
                {
                    if snake.segments[0].vel.1 != snake.base_speed || snake.segments.len() == 1
                    {
                        if snake.has_moved == false
                        {
                            snake.has_moved = true;
                            snake.segments[0].vel = (0.0, -snake.base_speed);
                        }

                        if (snake.segments[0].rect[0] - snake.segments[0].pos.0).abs() > SEGMENT_SIZE || (snake.segments[0].rect[1] - snake.segments[0].pos.1).abs() > SEGMENT_SIZE
                        {
                            snake.segments[0].vel = (0.0, -snake.base_speed);
                            snake.segments[0].pos = (snake.segments[0].rect[0], snake.segments[0].rect[1]);
                        }
                    }
                }
                Key::Down | Key::S =>
                {
                    if snake.segments[0].vel.1 != -snake.base_speed || snake.segments.len() == 1
                    {
                        if snake.has_moved == false
                        {
                            snake.has_moved = true;
                            snake.segments[0].vel = (0.0, snake.base_speed);
                        }
                        
                        if (snake.segments[0].rect[0] - snake.segments[0].pos.0).abs() > SEGMENT_SIZE || (snake.segments[0].rect[1] - snake.segments[0].pos.1).abs() > SEGMENT_SIZE
                        {
                            snake.segments[0].vel = (0.0, snake.base_speed);
                            snake.segments[0].pos = (snake.segments[0].rect[0], snake.segments[0].rect[1]);
                        }
                    }
                }
                Key::Left | Key::A =>
                {
                    if snake.segments[0].vel.0 != snake.base_speed || snake.segments.len() == 1
                    {
                        if snake.has_moved == false
                        {
                            snake.has_moved = true;
                            snake.segments[0].vel = (-snake.base_speed, 0.0);
                        }

                        if (snake.segments[0].rect[0] - snake.segments[0].pos.0).abs() > SEGMENT_SIZE || (snake.segments[0].rect[1] - snake.segments[0].pos.1).abs() > SEGMENT_SIZE
                        {
                            snake.segments[0].vel = (-snake.base_speed, 0.0);
                            snake.segments[0].pos = (snake.segments[0].rect[0], snake.segments[0].rect[1]);
                        }
                    }
                }
                Key::Right | Key::D =>
                {
                    if snake.segments[0].vel.0 != -snake.base_speed || snake.segments.len() == 1
                    {
                        if snake.has_moved == false
                        {
                            snake.has_moved = true;
                            snake.segments[0].vel = (snake.base_speed, 0.0);
                        }

                        if (snake.segments[0].rect[0] - snake.segments[0].pos.0).abs() > SEGMENT_SIZE || (snake.segments[0].rect[1] - snake.segments[0].pos.1).abs() > SEGMENT_SIZE
                        {
                            snake.segments[0].vel = (snake.base_speed, 0.0);
                            snake.segments[0].pos = (snake.segments[0].rect[0], snake.segments[0].rect[1]);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}