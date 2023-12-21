use crate::app_prefs::SoundSave;
use crate::theme::Theme;
use crate::ui::*;
use crate::Audio;
use indexmap::{indexmap, IndexMap};
use pixels_graphics_lib::buffer_graphics_lib::prelude::*;
use pixels_graphics_lib::prelude::*;
use usfx::{DutyCycle, OscillatorType, Sample};

pub struct Controller {
    pub items: IndexMap<Item, State>,
    pub audio: Audio,
    pub osc_type: OscillatorType,
    pub cycle: DutyCycle,
    pub shapes: ShapeCollection,
    pub button_shape: Drawable<Rect>,
    pub texts: Vec<Text>,
    pub duty_text: IndexMap<DutyCycle, Text>,
    pub osc_text: IndexMap<OscillatorType, Text>,
    pub waveform: Vec<Coord>,
    pub has_changed: bool
}

impl Controller {
    pub fn new(audio: Audio, theme: &Theme) -> Self {
        let items = indexmap! {
            Item::new('q', 'w', KeyCode::KeyQ, KeyCode::KeyW, "Volume") => State::Enabled(1.0),
            Item::new('a', 's', KeyCode::KeyA, KeyCode::KeyS, "Attack")=> State::Enabled(0.1),
            Item::new('z', 'x', KeyCode::KeyZ, KeyCode::KeyX, "Decay")=> State::Enabled(0.1),
            Item::new('e', 'r', KeyCode::KeyE, KeyCode::KeyR, "Sustain")=> State::Enabled(0.5),
            Item::new('d', 'f', KeyCode::KeyD, KeyCode::KeyF, "Release")=> State::Enabled(0.5),
            Item::new_int('c', 'v', KeyCode::KeyC, KeyCode::KeyV, "Freq")=> State::Enabled(500.0),
            Item::new_tog('t', 'y', 'u', KeyCode::KeyT, KeyCode::KeyY,KeyCode::KeyU, "Crunch")=> State::Disabled(0.0),
            Item::new_tog('g', 'h', 'j', KeyCode::KeyG, KeyCode::KeyH,KeyCode::KeyJ, "Drive")=> State::Disabled(0.0),
        };

        let (shapes, texts, osc_text, duty_text, button_shape) = Controller::gen_themed(theme);

        Controller {
            items,
            audio,
            shapes,
            button_shape,
            osc_type: OscillatorType::Sine,
            cycle: DutyCycle::Half,
            texts,
            duty_text,
            osc_text,
            waveform: vec![],
            has_changed: true
        }
    }
}

impl Controller {
    pub fn gen_themed(
        theme: &Theme,
    ) -> (
        ShapeCollection,
        Vec<Text>,
        IndexMap<OscillatorType, Text>,
        IndexMap<DutyCycle, Text>,
        Drawable<Rect>,
    ) {
        let shapes = generate_shapes(theme);

        let texts = generate_text(theme);
        let osc_text = osc_text(theme);
        let duty_text = duty_text(theme);

        let button_shape = Drawable::from_obj(Rect::new((0, 0), (11, 13)), stroke(theme.inactive));

        (shapes, texts, osc_text, duty_text, button_shape)
    }

    pub fn on_theme_change(&mut self, theme: &Theme) {
        let (shapes, texts, osc_text, duty_text, button_shape) = Controller::gen_themed(theme);
        self.shapes = shapes;
        self.texts = texts;
        self.osc_text = osc_text;
        self.duty_text = duty_text;
        self.button_shape = button_shape;
    }

    pub fn key_pressed(&mut self, key: KeyCode, shift_pressed: bool) {
        for (item, value) in self.items.iter_mut() {
            let mut delta = match item.item_type {
                ItemType::Float => 0.1,
                ItemType::Int => 10.0,
            };
            if shift_pressed {
                delta *= 10.0;
            }
            if key == item.dec_code {
                self.has_changed = true;
                if value.num() > delta {
                    *value = value.update(-delta);
                } else {
                    *value = value.replace(0.0);
                }
            }
            if key == item.inc_code {
                self.has_changed = true;
                *value = value.update(delta);
            }
            if let Some(tog) = item.toggle_code {
                if key == tog {
                    self.has_changed = true;
                    *value = value.swap();
                }
            }
        }
        if key == KeyCode::Digit1 {
            self.has_changed = true;
            self.osc_type = OscillatorType::Sine;
        } else if key == KeyCode::Digit2 {
            self.has_changed = true;
            self.osc_type = OscillatorType::Triangle;
        } else if key == KeyCode::Digit3 {
            self.has_changed = true;
            self.osc_type = OscillatorType::Saw;
        } else if key == KeyCode::Digit4 {
            self.has_changed = true;
            self.osc_type = OscillatorType::Square;
        } else if key == KeyCode::Digit5 {
            self.has_changed = true;
            self.osc_type = OscillatorType::Noise;
        }
        if key == KeyCode::Digit7 {
            self.has_changed = true;
            self.cycle = DutyCycle::Half;
        } else if key == KeyCode::Digit8 {
            self.has_changed = true;
            self.cycle = DutyCycle::Third;
        } else if key == KeyCode::Digit9 {
            self.has_changed = true;
            self.cycle = DutyCycle::Quarter;
        } else if key == KeyCode::Digit0 {
            self.has_changed = true;
            self.cycle = DutyCycle::Eight;
        }
        if key == KeyCode::Space {
            let sample = self.create_sample();
            self.audio.play(sample);
        }
    }

    pub fn create_save_data(&self) {
        let mut save = SoundSave {
            name: "".to_string(),
            when: Default::default(),
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
            osc: self.osc_type,
            duty: self.cycle,
        };
        for (item, value) in &self.items {
            match item.name {
                "Volume" => save.volume = value.num(),
                "Decay" => save.decay = value.num(),
                "Sustain" => save.sustain = value.num(),
                "Attack" => save.attack = value.num(),
                "Release" => save.release = value.num(),
                "Crunch" => {
                    save.crunch = value.num();
                    save.crunch_enabled = matches!(value, State::Enabled(_));
                }
                "Drive" => {
                    save.drive = value.num();
                    save.drive_enabled = matches!(value, State::Enabled(_));
                }
                "Freq" => save.freq = value.num() as u64,
                _ => {}
            }
        }
    }

    pub fn create_sample(&self) -> Sample {
        let mut sample = Sample::default();
        sample.osc_type(self.osc_type);
        sample.osc_duty_cycle(self.cycle);
        for (item, value) in &self.items {
            match item.name {
                "Volume" => {
                    sample.volume(value.num());
                }
                "Decay" => {
                    sample.env_decay(value.num());
                }
                "Sustain" => {
                    sample.env_sustain(value.num());
                }
                "Attack" => {
                    sample.env_attack(value.num());
                }
                "Release" => {
                    sample.env_release(value.num());
                }
                "Crunch" => {
                    if let State::Enabled(value) = value {
                        sample.dis_crunch(*value);
                    }
                }
                "Drive" => {
                    if let State::Enabled(value) = value {
                        sample.dis_drive(*value);
                    }
                }
                "Freq" => {
                    sample.osc_frequency(value.num() as usize);
                }
                _ => {}
            }
        }
        sample
    }

    pub fn render(&self, graphics: &mut Graphics<'_>, theme: &Theme, active_theme: usize) {
        render_ui(self, graphics, theme, active_theme, &self.waveform)
    }
}

pub enum State {
    Enabled(f32),
    Disabled(f32),
}

impl State {
    pub fn num(&self) -> f32 {
        *(match self {
            State::Enabled(num) => num,
            State::Disabled(num) => num,
        })
    }

    fn update(&mut self, value: f32) -> State {
        match self {
            State::Enabled(num) => State::Enabled(*num + value),
            State::Disabled(num) => State::Disabled(*num + value),
        }
    }

    fn replace(&mut self, value: f32) -> State {
        match self {
            State::Enabled(_) => State::Enabled(value),
            State::Disabled(_) => State::Disabled(value),
        }
    }

    fn swap(&mut self) -> State {
        match self {
            State::Enabled(num) => State::Disabled(*num),
            State::Disabled(num) => State::Enabled(*num),
        }
    }
}

#[derive(Hash, Debug, Eq, PartialEq)]
pub struct Item {
    pub dec: char,
    pub inc: char,
    pub toggle: Option<char>,
    pub dec_code: KeyCode,
    pub inc_code: KeyCode,
    pub toggle_code: Option<KeyCode>,
    pub name: &'static str,
    pub item_type: ItemType,
}

#[derive(Hash, Debug, Eq, PartialEq)]
pub enum ItemType {
    Float,
    Int,
}

impl Item {
    pub fn new(
        dec: char,
        inc: char,
        dec_code: KeyCode,
        inc_code: KeyCode,
        name: &'static str,
    ) -> Self {
        Self {
            dec,
            inc,
            dec_code,
            inc_code,
            name,
            item_type: ItemType::Float,
            toggle: None,
            toggle_code: None,
        }
    }

    pub fn new_tog(
        tog: char,
        dec: char,
        inc: char,
        tog_code: KeyCode,
        dec_code: KeyCode,
        inc_code: KeyCode,
        name: &'static str,
    ) -> Self {
        Self {
            dec,
            inc,
            dec_code,
            inc_code,
            name,
            item_type: ItemType::Float,
            toggle: Some(tog),
            toggle_code: Some(tog_code),
        }
    }

    pub fn new_int(
        dec: char,
        inc: char,
        dec_code: KeyCode,
        inc_code: KeyCode,
        name: &'static str,
    ) -> Self {
        Self {
            dec,
            inc,
            dec_code,
            inc_code,
            name,
            item_type: ItemType::Int,
            toggle: None,
            toggle_code: None,
        }
    }
}
