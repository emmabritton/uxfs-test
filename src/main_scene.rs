use crate::audio::SAMPLE_RATE;
use crate::theme::{themes, Theme};
use crate::waveform::Waveform;
use crate::*;
use pixels_graphics_lib::buffer_graphics_lib::Graphics;
use pixels_graphics_lib::prelude::SceneUpdateResult::Nothing;
use pixels_graphics_lib::prelude::*;
use pixels_graphics_lib::scenes::SceneUpdateResult::Pop;
use usfx::{Mixer, Sample};

pub struct MainScene {
    controller: Controller,
    result: SceneUpdateResult<SR, SN>,
    next_input: f64,
    prefs: AppPrefs<Settings>,
    themes: Vec<Theme>,
}

impl MainScene {
    pub fn new(mut prefs: AppPrefs<Settings>) -> MainScene {
        let mut audio = Audio::new();
        audio.run();
        let themes = themes();
        prefs.data.theme = prefs.data.theme.min(themes.len() - 1);
        MainScene {
            controller: Controller::new(audio, &themes[prefs.data.theme]),
            result: Nothing,
            next_input: 0.0,
            prefs,
            themes,
        }
    }
}

impl MainScene {
    fn save_sound(&mut self, idx: usize) {
        self.prefs.data.saved[idx] = Some(self.controller.create_save_data());
    }

    fn load_sound(&mut self, idx: usize) {
        if let Some(sound) = &self.prefs.data.saved[idx] {
            self.controller.load(sound);
        }
    }

    fn delete_sound(&mut self, idx: usize) {
        self.prefs.data.saved[idx] = None;
    }
}

impl Scene<SR, SN> for MainScene {
    fn render(&self, graphics: &mut Graphics, _: &MouseData, _: &FxHashSet<KeyCode>) {
        let theme = &self.themes[self.prefs.data.theme];
        graphics.clear(theme.background);
        self.controller.render(
            graphics,
            theme,
            self.prefs.data.theme,
            &self.prefs.data.saved,
        );
    }

    fn on_key_down(&mut self, key: KeyCode, _: &MouseData, held_keys: &FxHashSet<KeyCode>) {
        if is_modifier(key) {
            return;
        }
        if self.next_input <= 0.0 {
            self.next_input = 0.5;
            if key == KeyCode::ArrowLeft {
                self.prefs.data.theme = self.prefs.data.theme.saturating_sub(1);
                self.controller
                    .on_theme_change(&self.themes[self.prefs.data.theme]);
                return;
            }
            if key == KeyCode::ArrowRight {
                self.prefs.data.theme = (self.prefs.data.theme + 1).min(self.themes.len() - 1);
                self.controller
                    .on_theme_change(&self.themes[self.prefs.data.theme]);
                return;
            }
            if matches!(
                key,
                KeyCode::Digit1
                    | KeyCode::Digit2
                    | KeyCode::Digit3
                    | KeyCode::Digit4
                    | KeyCode::Digit5
                    | KeyCode::Digit6
                    | KeyCode::Digit7
                    | KeyCode::Digit8
                    | KeyCode::Digit9
                    | KeyCode::Digit0
            ) {
                let idx = func_key_idx(key);
                if held_keys.contains(&KeyCode::ControlLeft)
                    || held_keys.contains(&KeyCode::ControlRight)
                {
                    self.delete_sound(idx);
                } else if held_keys.contains(&KeyCode::ShiftLeft)
                    || held_keys.contains(&KeyCode::ShiftRight)
                {
                    self.load_sound(idx);
                } else {
                    self.save_sound(idx);
                }
            }
            self.controller.key_pressed(
                key,
                held_keys.contains(&KeyCode::ShiftLeft) || held_keys.contains(&KeyCode::ShiftRight),
                held_keys.contains(&KeyCode::ControlLeft)
                    || held_keys.contains(&KeyCode::ControlRight),
            );
        }
        if key == KeyCode::Escape {
            self.prefs.save();
            self.result = Pop(None);
        }
    }

    fn update(
        &mut self,
        timing: &Timing,
        _: &MouseData,
        _: &FxHashSet<KeyCode>,
        _: &Window,
    ) -> SceneUpdateResult<SR, SN> {
        if self.controller.has_changed {
            self.controller.has_changed = false;
            let sample = self.controller.create_sample();
            let data = convert_to_data(sample);
            self.controller.waveform = Waveform::new(data, SAMPLE_RATE as usize, 334, 42);
        }
        self.next_input -= timing.fixed_time_step;
        self.result.clone()
    }

    fn resuming(&mut self, _: Option<SR>) {}
}

fn convert_to_data(sample: Sample) -> Vec<f32> {
    let mut mixer = Mixer::new(SAMPLE_RATE as usize);
    mixer.play(sample);
    let mut output = vec![];
    let mut buffer = [0.0; 100];
    loop {
        mixer.generate(&mut buffer);
        if buffer.iter().any(|&num| num != 0.0 && num != -0.0) {
            output.extend_from_slice(
                &buffer
                    .iter()
                    .copied()
                    .filter(|v| v.is_normal())
                    .collect::<Vec<f32>>(),
            );
        } else {
            break;
        }
    }
    output
}

fn func_key_idx(key: KeyCode) -> usize {
    match key {
        KeyCode::Digit1 => 0,
        KeyCode::Digit2 => 1,
        KeyCode::Digit3 => 2,
        KeyCode::Digit4 => 3,
        KeyCode::Digit5 => 4,
        KeyCode::Digit6 => 5,
        KeyCode::Digit7 => 6,
        KeyCode::Digit8 => 7,
        KeyCode::Digit9 => 8,
        KeyCode::Digit0 => 9,
        _ => panic!("Invalid key code {key:?}"),
    }
}

fn is_modifier(key: KeyCode) -> bool {
    matches!(
        key,
        KeyCode::ShiftRight
            | KeyCode::ShiftLeft
            | KeyCode::ControlLeft
            | KeyCode::ControlRight
            | KeyCode::AltLeft
            | KeyCode::AltRight
    )
}
