extern crate sdl2;
use std::time::Duration;
use sdl2::rect::Rect;
use models::state;

mod gen_loop;
mod config;
mod models;
mod gen_render;
mod gen_logic;
mod render;
mod logic;
mod nn;
mod gen_events;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Gen programming", config::WINDOW_WIDTH, config::WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())?;

    canvas.set_viewport(Rect::new(
        (config::WINDOW_WIDTH - config::CANVAS_WIDTH) as i32 / 2,
        (config::WINDOW_HEIGHT - config::CANVAS_HEIGHT) as i32 / 2,
        config::CANVAS_WIDTH,
        config::CANVAS_HEIGHT,
    ));


    let mut event_pump = sdl_context.event_pump()?;
    let mut state = state::State::new(canvas);

    gen_loop::init(&mut state);

    'main: loop {
        gen_events::process_event(&mut event_pump, &mut state);

        state.canvas.set_draw_color(config::BACKGROUND_COLOR);
        state.canvas.clear();
        state.canvas.set_draw_color(config::VIEWPORT_COLOR);
        state.canvas.fill_rect(Rect::new(
            0,
            0,
            config::CANVAS_WIDTH,
            config::CANVAS_HEIGHT,
        )).unwrap();
        gen_loop::run(&mut state);
        state.canvas.present();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / config::FPS));
        if state.exit {
            break 'main;
        }
    }

    Ok(())
}