mod common;
mod models;

extern crate dotenv;

use models::game::Game;
use models::sound::Sound;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mixer::{InitFlag, AUDIO_S32LSB, DEFAULT_CHANNELS};
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::render::BlendMode;

use std::time::Duration;

static WIDTH: u32 = 800;
static HEIGHT: u32 = 800;

fn main() {
    let sdl_context = sdl2::init().unwrap();

    // ------------------------------------------
    // ------------ VIDEO COMPONENTS ------------
    // ------------------------------------------

    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("DRW Chess", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");

    canvas.set_blend_mode(BlendMode::Blend);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let renderer = canvas.texture_creator();

    // ------------------------------------------
    // ------------ AUDIO COMPONENTS ------------
    // ------------------------------------------

    let _audio = sdl_context.audio().unwrap();

    let frequency = 44_100;
    let format = AUDIO_S32LSB; // signed 32 bit samples, in little-endian byte order
    let channels = DEFAULT_CHANNELS; // Stereo
    let chunk_size = 1_024;
    sdl2::mixer::open_audio(frequency, format, channels, chunk_size).unwrap();
    let _mixer_context = sdl2::mixer::init(InitFlag::MP3).unwrap();
    // Number of mixing channels available for sound effect 'Chunk's to play
    // simultaneously
    sdl2::mixer::allocate_channels(4);

    // ------------------------------------------
    // ------------ GAME COMPONENTS -------------
    // ------------------------------------------

    let mut game: Game = Game::new(&renderer);
    let sound: Sound = Sound::new();
    sound.play("starting_game");

    let mut mouse_x: i32 = 0;
    let mut mouse_y: i32 = 0;

    // ------------------------------------------
    // --------------- MAIN LOOP ----------------
    // ------------------------------------------

    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    x,
                    y,
                    ..
                } => {
                    game.select_piece(x, y, WIDTH, HEIGHT);
                }
                Event::MouseButtonUp {
                    mouse_btn: MouseButton::Left,
                    x,
                    y,
                    ..
                } => {
                    game.make_move(x, y, WIDTH, HEIGHT, &sound);
                }
                Event::MouseMotion { x, y, .. } => {
                    mouse_x = x;
                    mouse_y = y;
                }
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
        canvas.clear();
        game.draw(&mut canvas, WIDTH as i32, HEIGHT as i32, mouse_x, mouse_y);

        canvas.present();
        // Time management
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
