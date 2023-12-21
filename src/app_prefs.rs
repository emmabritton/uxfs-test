use anyhow::Result;
use chrono::{DateTime, Utc};
use pixels_graphics_lib::prefs::preferences::{get_pref_dir, Preferences};
use serde::{Deserialize, Serialize};
use usfx::{DutyCycle, OscillatorType};

const KEY: &str = "user.settings";

pub struct AppPrefs {
    prefs: Preferences<Settings>,
    pub data: Settings,
}

impl AppPrefs {
    pub fn new() -> Result<Self> {
        let mut prefs: Preferences<Settings> = Preferences::new(
            get_pref_dir("app", "emmabritton", "usfx_tester")?,
            "user.pref",
        );
        if let Err(e) = prefs.load() {
            eprintln!("Unable to restore app prefs: {e:?}");
        }
        let data: Settings = if let Some(data) = prefs.get(KEY) {
            data.clone()
        } else {
            Settings {
                theme: 0,
                saved: vec![],
            }
        };
        Ok(AppPrefs { prefs, data })
    }
}

impl AppPrefs {
    pub fn save(&mut self) {
        self.prefs.set(KEY, self.data.clone());
        if let Err(e) = self.prefs.save() {
            eprintln!("Unable to save app prefs: {e:?}");
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub theme: usize,
    pub saved: Vec<SoundSave>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundSave {
    pub name: String,
    pub when: DateTime<Utc>,
    pub volume: f32,
    pub attack: f32,
    pub decay: f32,
    pub sustain: f32,
    pub release: f32,
    pub freq: u64,
    pub crunch: f32,
    pub crunch_enabled: bool,
    pub drive: f32,
    pub drive_enabled: bool,
    pub osc: OscillatorType,
    pub duty: DutyCycle,
}

impl SoundSave {
    pub fn new(name: String) -> Self {
        Self {
            name,
            when: Utc::now(),
            volume: 0.0,
            attack: 0.0,
            decay: 0.0,
            sustain: 0.0,
            release: 0.0,
            freq: 0,
            crunch: 0.0,
            crunch_enabled: false,
            drive: 0.0,
            drive_enabled: false,
            osc: OscillatorType::Sine,
            duty: DutyCycle::Half,
        }
    }
}

impl SoundSave {
    pub fn freq(&self) -> usize {
        usize::try_from(self.freq).unwrap_or_else(|_| 500)
    }
}
