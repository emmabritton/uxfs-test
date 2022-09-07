#![windows_subsystem = "windows"]

mod audio;
mod controller;
mod ui;

use crate::audio::Audio;
use crate::controller::Controller;
use anyhow::Result;
use buffer_graphics_lib::color::BLACK;
use buffer_graphics_lib::Graphics;
use pixels_graphics_lib::prefs::WindowPreferences;
use pixels_graphics_lib::{setup, WindowScaling};
use std::thread::sleep;
use std::time::Duration;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;

const WIDTH: usize = 260;
const HEIGHT: usize = 320;

fn main() -> Result<()> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let (mut window, mut pixels) = setup(
        (WIDTH, HEIGHT),
        WindowScaling::Auto,
        "USFX Test",
        &event_loop,
    )?;
    let mut prefs = WindowPreferences::new("app", "raybritton", "usfx_tester")?;
    prefs.load()?;
    prefs.restore(&mut window);

    let mut audio = Audio::new();
    audio.run();
    let mut basic = Controller::new(audio);

    event_loop.run(move |event, _, control_flow| {
        if let Event::LoopDestroyed = event {
            prefs.store(&window);
            //can't return from here so just print out error
            let _ = prefs
                .save()
                .map_err(|err| eprintln!("Unable to save prefs: {:?}", err));
        }

        if let Event::RedrawRequested(_) = event {
            let mut graphics = Graphics::new(pixels.get_frame(), WIDTH, HEIGHT).unwrap();
            graphics.clear(BLACK);
            basic.render(&mut graphics);
            if pixels
                .render()
                .map_err(|e| eprintln!("pixels.render() failed: {:?}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            basic.input(&input);

            window.request_redraw();
        }

        sleep(Duration::from_millis(1));
    });
}
