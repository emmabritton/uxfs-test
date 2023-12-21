#![windows_subsystem = "windows"]

mod settings;
mod audio;
mod controller;
mod main_scene;
mod theme;
mod ui;
mod waveform;

use crate::audio::Audio;
use crate::controller::Controller;
use crate::main_scene::MainScene;
use anyhow::Result;
use log::LevelFilter;
use pixels_graphics_lib::prelude::*;
use crate::settings::Settings;

const WIDTH: usize = 340;
const HEIGHT: usize = 370;

fn main() -> Result<()> {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Warn)
        .filter_module("uxfs-test", LevelFilter::Trace)
        .format_timestamp(None)
        .format_module_path(false)
        .format_level(false)
        .init();

    let window_prefs = WindowPreferences::new("app", "emmabritton", "usfx_tester", 2)?;
    let app_prefs: AppPrefs<Settings> = AppPrefs::new("app", "emmabritton", "usfx_tester", || Settings::default())?;
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
