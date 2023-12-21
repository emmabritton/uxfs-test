#![windows_subsystem = "windows"]

mod app_prefs;
mod audio;
mod controller;
mod main_scene;
mod theme;
mod ui;
mod waveform;

use crate::app_prefs::AppPrefs;
use crate::audio::Audio;
use crate::controller::Controller;
use crate::main_scene::MainScene;
use anyhow::Result;
use pixels_graphics_lib::prelude::*;

const WIDTH: usize = 340;
const HEIGHT: usize = 370;

fn main() -> Result<()> {
    let window_prefs = WindowPreferences::new("app", "emmabritton", "usfx_tester")?;
    let app_prefs = AppPrefs::new()?;
    let system = Box::new(MainScene::new(app_prefs));
    run_scenes(
        WIDTH,
        HEIGHT,
        "USFX Test",
        Some(window_prefs),
        |_style, _list, _key| {},
        system,
        Options::default(),
    )?;
    Ok(())
}

//unused for this app but needed by scene system
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum SR {}

//unused for this app but needed by scene system
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum SN {}
