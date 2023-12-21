use crate::theme::{themes, Theme};
use crate::*;
use pixels_graphics_lib::buffer_graphics_lib::color::CYAN;
use pixels_graphics_lib::buffer_graphics_lib::prelude::TextPos::Px;
use pixels_graphics_lib::buffer_graphics_lib::prelude::TextSize::Normal;
use pixels_graphics_lib::buffer_graphics_lib::prelude::{Coord, BLACK};
use pixels_graphics_lib::buffer_graphics_lib::Graphics;
use pixels_graphics_lib::prelude::SceneUpdateResult::Nothing;
use pixels_graphics_lib::prelude::*;
use pixels_graphics_lib::scenes::SceneUpdateResult::Pop;
use usfx::{Mixer, Sample};
use crate::waveform::to_waveform;

pub struct MainScene {
    controller: Controller,
    result: SceneUpdateResult<SR, SN>,
    next_input: f64,
    prefs: AppPrefs,
    themes: Vec<Theme>,
}

impl MainScene {
    pub fn new(mut prefs: AppPrefs) -> MainScene {
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

impl Scene<SR, SN> for MainScene {
    fn render(&self, graphics: &mut Graphics, _: Coord) {
        let theme = &self.themes[self.prefs.data.theme];
        graphics.clear(theme.background);
        self.controller
            .render(graphics, theme, self.prefs.data.theme);
    }

    fn on_key_down(&mut self, key: KeyCode, _: Coord, held_keys: &Vec<&KeyCode>) {
        if self.next_input <= 0.0 {
            self.next_input = 0.7;
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
            self.controller.key_pressed(
                key,
                held_keys.contains(&&KeyCode::ShiftLeft)
                    || held_keys.contains(&&KeyCode::ShiftRight),
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
        _: Coord,
        _: &Vec<&KeyCode>,
    ) -> SceneUpdateResult<SR, SN> {
        if self.controller.has_changed {
            self.controller.has_changed = false;
            let sample = self.controller.create_sample();
            let data = convert_to_data(sample);
            self.controller.waveform = to_waveform(data, 335, 42);
        }
        self.next_input -= timing.fixed_time_step;
        self.result.clone()
    }

    fn resuming(&mut self, _: Option<SR>) {}
}

fn convert_to_data(sample: Sample) -> Vec<f32> {
    let mut mixer = Mixer::new(44100);
    mixer.play(sample);
    let mut output = vec![];
    let mut buffer = [0.0; 100];
    loop {
        mixer.generate(&mut buffer);
        if buffer.iter().any(|&num| num > 0.0 || num < 0.0) {
            output.extend_from_slice(&buffer);
        } else {
            break;
        }
    }
    output
}