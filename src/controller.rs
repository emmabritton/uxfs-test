use crate::settings::SoundSave;
use crate::theme::Theme;
use crate::ui::*;
use crate::waveform::Waveform;
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
    pub waveform: Waveform,
    pub has_changed: bool,
}

impl Controller {
    pub fn new(audio: Audio, theme: &Theme) -> Self {
        let items = indexmap! {
            Item::new('Q', 'W', KeyCode::KeyQ, KeyCode::KeyW, ITEM_VOLUME) => State::Enabled(1.0),
            Item::new('A', 'S', KeyCode::KeyA, KeyCode::KeyS, ITEM_ATTACK)=> State::Enabled(0.1),
            Item::new('Z', 'X', KeyCode::KeyZ, KeyCode::KeyX, ITEM_DECAY)=> State::Enabled(0.1),
            Item::new('E', 'R', KeyCode::KeyE, KeyCode::KeyR, ITEM_SUSTAIN)=> State::Enabled(0.5),
            Item::new('D', 'F', KeyCode::KeyD, KeyCode::KeyF, ITEM_RELEASE)=> State::Enabled(0.5),
            Item::new_int('C', 'V', KeyCode::KeyC, KeyCode::KeyV, ITEM_FREQ)=> State::Enabled(500.0),
            Item::new_tog('T', 'Y', 'U', KeyCode::KeyT, KeyCode::KeyY,KeyCode::KeyU, ITEM_CRUNCH)=> State::Disabled(0.0),
            Item::new_tog('G', 'H', 'J', KeyCode::KeyG, KeyCode::KeyH,KeyCode::KeyJ, ITEM_DRIVE)=> State::Disabled(0.0),
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
            waveform: Waveform::new(vec![], 1, 1, 1),
            has_changed: true,
        }
    }
}

impl Controller {
    #[allow(clippy::type_complexity)]
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
        if key == KeyCode::KeyI {
            self.has_changed = true;
            self.osc_type = OscillatorType::Sine;
        } else if key == KeyCode::KeyO {
            self.has_changed = true;
            self.osc_type = OscillatorType::Triangle;
        } else if key == KeyCode::KeyP {
            self.has_changed = true;
            self.osc_type = OscillatorType::Saw;
        } else if key == KeyCode::KeyK {
            self.has_changed = true;
            self.osc_type = OscillatorType::Square;
        } else if key == KeyCode::KeyL {
            self.has_changed = true;
            self.osc_type = OscillatorType::Noise;
        }
        if key == KeyCode::KeyB {
            self.has_changed = true;
            self.cycle = DutyCycle::Half;
        } else if key == KeyCode::KeyN {
            self.has_changed = true;
            self.cycle = DutyCycle::Third;
        } else if key == KeyCode::KeyM {
            self.has_changed = true;
            self.cycle = DutyCycle::Quarter;
        } else if key == KeyCode::Comma {
            self.has_changed = true;
            self.cycle = DutyCycle::Eight;
        }
        if key == KeyCode::Space {
            let sample = self.create_sample();
            self.audio.play(sample);
        }
    }

    pub fn load(&mut self, sound: &SoundSave) {
        for (item, value) in self.items.iter_mut() {
            match item.name {
                ITEM_VOLUME => {
                    value.replace(sound.volume);
                }
                ITEM_ATTACK => {
                    value.replace(sound.attack);
                }
                ITEM_DECAY => {
                    value.replace(sound.decay);
                }
                ITEM_SUSTAIN => {
                    value.replace(sound.sustain);
                }
                ITEM_RELEASE => {
                    value.replace(sound.release);
                }
                ITEM_CRUNCH => {
                    *value = if sound.crunch_enabled {
                        State::Enabled(sound.crunch)
                    } else {
                        State::Disabled(sound.crunch)
                    };
                }
                ITEM_DRIVE => {
                    *value = if sound.drive_enabled {
                        State::Enabled(sound.drive)
                    } else {
                        State::Disabled(sound.drive)
                    };
                }
                ITEM_FREQ => {
                    value.replace(sound.freq() as f32);
                }
                _ => {}
            }
            self.has_changed = true;
        }
    }

    pub fn create_save_data(&self) -> SoundSave {
        let mut save = SoundSave::new_blank();
        for (item, value) in &self.items {
            match item.name {
                ITEM_VOLUME => save.volume = value.num(),
                ITEM_DECAY => save.decay = value.num(),
                ITEM_SUSTAIN => save.sustain = value.num(),
                ITEM_ATTACK => save.attack = value.num(),
                ITEM_RELEASE => save.release = value.num(),
                ITEM_CRUNCH => {
                    save.crunch = value.num();
                    save.crunch_enabled = matches!(value, State::Enabled(_));
                }
                ITEM_DRIVE => {
                    save.drive = value.num();
                    save.drive_enabled = matches!(value, State::Enabled(_));
                }
                ITEM_FREQ => save.freq = value.num() as u64,
                _ => {}
            }
        }
        save.osc = self.osc_type;
        save.duty = self.cycle;
        save.fix_name();
        save
    }

    pub fn create_sample(&self) -> Sample {
        let mut sample = Sample::default();
        sample.osc_type(self.osc_type);
        sample.osc_duty_cycle(self.cycle);
        for (item, value) in &self.items {
            match item.name {
                ITEM_VOLUME => {
                    sample.volume(value.num());
                }
                ITEM_DECAY => {
                    sample.env_decay(value.num());
                }
                ITEM_SUSTAIN => {
                    sample.env_sustain(value.num());
                }
                ITEM_ATTACK => {
                    sample.env_attack(value.num());
                }
                ITEM_RELEASE => {
                    sample.env_release(value.num());
                }
                ITEM_CRUNCH => {
                    if let State::Enabled(value) = value {
                        sample.dis_crunch(*value);
                    }
                }
                ITEM_DRIVE => {
                    if let State::Enabled(value) = value {
                        sample.dis_drive(*value);
                    }
                }
                ITEM_FREQ => {
                    sample.osc_frequency(value.num() as usize);
                }
                _ => {}
            }
        }
        sample
    }

    pub fn render(
        &self,
        graphics: &mut Graphics<'_>,
        theme: &Theme,
        active_theme: usize,
        saves: &[Option<SoundSave>],
    ) {
        render_ui(self, graphics, theme, active_theme, &self.waveform, saves)
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

const ITEM_VOLUME: &str = "volume";
const ITEM_ATTACK: &str = "attack";
const ITEM_DECAY: &str = "decay";
const ITEM_SUSTAIN: &str = "sustain";
const ITEM_RELEASE: &str = "release";
const ITEM_FREQ: &str = "freq";
const ITEM_CRUNCH: &str = "crunch";
const ITEM_DRIVE: &str = "drive";
