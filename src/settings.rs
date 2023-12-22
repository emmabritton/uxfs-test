use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use usfx::{DutyCycle, OscillatorType};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Settings {
    pub theme: usize,
    pub saved: [Option<SoundSave>; 9],
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
    pub fn new_blank() -> Self {
        Self {
            name: String::new(),
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

    pub fn fix_name(&mut self) {
        let osc = match self.osc {
            OscillatorType::Sine => "sin",
            OscillatorType::Saw => "saw",
            OscillatorType::Triangle => "tri",
            OscillatorType::Square => "squ",
            OscillatorType::Noise => "noi",
        };
        let crunch = match self.crunch_enabled {
            false => " - ".to_string(),
            true => format!("{:.1}", self.crunch),
        };
        let drive = match self.drive_enabled {
            false => " - ".to_string(),
            true => format!("{:.1}", self.drive),
        };
        let duty = match self.duty {
            DutyCycle::Eight => "1/8",
            DutyCycle::Quarter => "1/4",
            DutyCycle::Third => "1/3",
            DutyCycle::Half => "1/2",
        };
        self.name = format!(
            "{} {: >4} {:.1} {:.1} {:.1} {:.1} {} {} {}",
            osc, self.freq, self.attack, self.decay, self.sustain, self.release, crunch, drive, duty
        )
    }

    pub fn formatted_when(&self) -> String {
        self.when.format("%Y/%m/%d %H:%M").to_string()
    }
}

impl SoundSave {
    pub fn freq(&self) -> usize {
        usize::try_from(self.freq).unwrap_or(500)
    }
}
