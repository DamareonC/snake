# Snake

Snake is a simple game I made using Rust and the Piston game engine. In the game, you play as a snake, and the goal is to eat as many apples as you can. Every time you eat an apple, you become longer. If you run into yourself, or if you hit the edge of the screen, then it is game over.

## Requirements

To build and run Snake, you first need to download [Rust](https://doc.rust-lang.org/stable/book/ch01-01-installation.html) and [Git](https://git-scm.com/downloads).

## Running Snake

To run Snake:
1. Open a terminal
2. Clone this repo with `git clone https://github.com/DamareonC/snake.git`
3. Go into the directory with `cd snake`
4. Build and start Snake with `cargo run --release` (this may take a while since Snake's dependencies have to be compiled)

To run again, either rerun `cargo run --release` or run the executable (called *snake* in MacOS and Linux and *snake.exe* on Windows) that is located in /target/release directory.
