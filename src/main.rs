mod apple;
mod snake;
mod game;

use std::time::Instant;
use rand::Rng;
use piston::{input::RenderEvent, window::WindowSettings, EventSettings, Events, PressEvent, UpdateEvent};
use graphics::rectangle;
use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL, GlGraphics};

use crate::{game::Game, snake::{Snake, SnakeSegment}, apple::Apple};

const WINDOW_WIDTH: f64 = 512.0;
const WINDOW_HEIGHT: f64 = 512.0;
const SEGMENT_SIZE: f64 = 20.0;
const MAX_WIDTH_RANGE: i16 = (WINDOW_WIDTH - SEGMENT_SIZE) as i16;
const MAX_HEIGHT_RANGE: i16 = (WINDOW_HEIGHT - SEGMENT_SIZE) as i16;

fn main()
{
    let opengl: OpenGL = OpenGL::V4_5;
    let mut window: GlutinWindow = WindowSettings::new("Snake", [WINDOW_WIDTH, WINDOW_HEIGHT]).graphics_api(opengl).exit_on_esc(true).build().unwrap();
    let mut game: Game = Game
    {
        gl: GlGraphics::new(opengl),
        score: 0,
        game_over: false
    };
    let initial_pos: (f64, f64) = (rand::thread_rng().gen_range(0..=MAX_WIDTH_RANGE) as f64, rand::thread_rng().gen_range(0..=MAX_HEIGHT_RANGE) as f64);
    let mut snake: Snake = Snake
    {
        segments: vec![SnakeSegment {
            rect: rectangle::square(initial_pos.0, initial_pos.1, SEGMENT_SIZE),
            pos: initial_pos,
            vel: (0.0, 0.0)
        }],
        base_speed: 2.0,
        has_moved: false
    };
    let mut apple: Apple = Apple
    {
        rect: rectangle::square(rand::thread_rng().gen_range(0..=MAX_WIDTH_RANGE) as f64, rand::thread_rng().gen_range(0..=MAX_HEIGHT_RANGE) as f64, 20.0)
    };
    let mut events: Events = Events::new(EventSettings::new());

    let mut last_time: Instant = Instant::now();
    let ns_per_update: f64 = 1_000_000_000.0 / 60.0;
    let mut delta: f64 = 0.0;

    while let Some(e) = events.next(&mut window)
    {
        if let Some(r) = e.render_args()
        {
            game.render(&snake, &apple, r);
        }
        if let Some(u) = e.update_args()
        {
            let current_time: Instant = Instant::now();

            delta += current_time.duration_since(last_time).as_nanos() as f64 / ns_per_update;
            last_time = current_time;

            while delta >= 1.0
            {
                game.update(&mut snake, &mut apple, u);
                delta -= 1.0;
            }
        }
        if let Some(p) = e.press_args()
        {
            game.key_pressed(&mut snake, p);
        }
        if game.game_over
        {
            break;
        }
    };
}
