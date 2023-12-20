#![windows_subsystem = "windows"]

mod audio;
mod controller;
mod main_scene;
mod ui;

use crate::audio::Audio;
use crate::controller::Controller;
use crate::main_scene::MainScene;
use anyhow::Result;
use pixels_graphics_lib::prelude::*;

const WIDTH: usize = 260;
const HEIGHT: usize = 320;

fn main() -> Result<()> {
    let prefs = WindowPreferences::new("app", "emmabritton", "usfx_tester")?;
    let system = Box::new(MainScene::new());
    run_scenes(
        WIDTH,
        HEIGHT,
        "USFX Test",
        Some(prefs),
        |_style, _list, _key| {},
        system,
        Options::default(),
    )?;
    Ok(())
}

//unused for this app but needed by scene system
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum SR {}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum SN {}
