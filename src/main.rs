use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
// use sdl2::render::WindowCanvas;
use std::time::Duration;

mod render;
mod update;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("jiricodes - sdl2 sandbox", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    render::render(&mut canvas, Color::RGB(0, 255, 255));
    let mut event_pump = sdl_context.event_pump()?;
	let mut i: u8 = 0;
	let mut increase: bool = true;
    'running: loop {
		// Handle Events
		for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }
		// Update
		if i == 0 {
			increase = true;
		} else if i == 255 {
			increase = false;
		}
		i = update::update(i, increase);
		// Render
		render::render(&mut canvas, Color::RGB(i, 64, 255 - i));
        // The rest of the game loop goes here...
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
