mod models;
mod common;

use models::board::Board;
use models::game::Game;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::mouse::MouseButton;

use std::time::Duration;

static WIDTH: u32 = 800;
static HEIGHT: u32 = 800;

fn render(canvas: &mut WindowCanvas) {
    canvas.present();
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("DRW Chess", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build().expect("could not make canvas");

    let mut event_pump = sdl_context.event_pump().unwrap();
    let renderer = canvas.texture_creator();

    let mut game: Game = Game::new(&renderer);

    let mut mouse_x: i32 = 0;
    let mut mouse_y: i32 = 0;

    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown {
                    keycode: Some(Keycode::Escape), ..
                } => {
                    break 'running;
                },
                Event::MouseButtonDown {mouse_btn: MouseButton::Left, x:x, y:y, ..} => {
                    game.select_piece(x, y, WIDTH, HEIGHT);
                },
                Event::MouseButtonUp {mouse_btn: MouseButton::Left, x:x, y:y, ..} => {
                    game.make_move(x, y, WIDTH, HEIGHT);
                },
                Event::MouseMotion{x, y, ..} => {
                    mouse_x = x;
                    mouse_y = y;
                },
                _ => {}
                
            }
        }

        game.draw(&mut canvas, WIDTH as i32, HEIGHT as i32, mouse_x, mouse_y);

        canvas.present();
        // Time management
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
