use ndarray::{ArrayViewMut, Ix3};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use std::time::{Duration, Instant};

mod array_helper;
mod color;

pub use color::Image;
pub mod util {
    pub use crate::array_helper::*;
    pub use crate::color::*;
}

pub struct Config {
    pub width: u32,
    pub height: u32,
    pub print_framerate: bool,
}

pub struct Inputs {
    pub events: Vec<sdl2::event::Event>,
}

pub fn run<F>(config: Config, mut callback: F) -> Result<(), String>
where
    F: for<'a> FnMut(color::Image, &Inputs, (u32, u32)),
{
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", config.width, config.height)
        .position_centered()
        .resizable()
        //.borderless()
        //.maximized()
        //.fullscreen()
        .build()
        .unwrap();

    let (mut width, mut height) = window.size();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB888, width, height)
        .unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut inputs = Inputs { events: Vec::new() };
    let mut num_frames_this_second = 0;
    let mut start_time = Instant::now();
    'running: loop {
        let new_width = canvas.output_size().unwrap().0;
        let new_height = canvas.output_size().unwrap().1;
        if new_height != height || new_width != width {
            width = new_width;
            height = new_height;
            texture = texture_creator
                .create_texture_streaming(PixelFormatEnum::RGB888, width, height)
                .unwrap();
        }
        inputs.events.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => inputs.events.push(event),
            }
        }
        let mut callback_wrapper = |pixels: &mut [u8], pitch: usize| {
            // Strange that it still gives us four channels despite the RGB pixel format
            assert!(pitch == width as usize * 4);
            let arr =
                ArrayViewMut::from_shape((height as usize, width as usize, 4), pixels).unwrap();
            callback(color::Image::from(arr), &inputs, (width, height));
        };
        texture
            .with_lock(None, &mut callback_wrapper)
            .expect("Erorr in with_lock");
        canvas.clear();
        canvas.copy(&texture, None, None)?;
        canvas.present();
        num_frames_this_second += 1;
        let elapsed = start_time.elapsed();
        if elapsed >= Duration::from_secs(1) {
            start_time = Instant::now();
            println!(
                "{} fps",
                num_frames_this_second as f32 / elapsed.as_secs_f32()
            );
            num_frames_this_second = 0;
        }
        // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
