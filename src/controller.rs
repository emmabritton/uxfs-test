use crate::ui::{duty_text, generate_shapes, generate_text, osc_text, render_ui};
use crate::Audio;
use buffer_graphics_lib::color::LIGHT_GRAY;
use buffer_graphics_lib::shapes::{stroke, Shape};
use buffer_graphics_lib::text::Text;
use buffer_graphics_lib::Graphics;
use indexmap::{indexmap, IndexMap};
use usfx::{DutyCycle, OscillatorType, Sample};
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

pub struct Controller {
    pub items: IndexMap<Item, State>,
    pub audio: Audio,
    pub osc_type: OscillatorType,
    pub cycle: DutyCycle,
    pub shapes: Vec<Shape>,
    pub button_shape: Shape,
    pub texts: Vec<Text>,
    pub duty_text: IndexMap<DutyCycle, Text>,
    pub osc_text: IndexMap<OscillatorType, Text>,
}

impl Controller {
    pub fn new(audio: Audio) -> Self {
        let items = indexmap! {
            Item::new('q', 'w', VirtualKeyCode::Q, VirtualKeyCode::W, "Volume") => State::Enabled(1.0),
            Item::new('a', 's', VirtualKeyCode::A, VirtualKeyCode::S, "Attack")=> State::Enabled(0.1),
            Item::new('z', 'x', VirtualKeyCode::Z, VirtualKeyCode::X, "Decay")=> State::Enabled(0.1),
            Item::new('e', 'r', VirtualKeyCode::E, VirtualKeyCode::R, "Sustain")=> State::Enabled(0.5),
            Item::new('d', 'f', VirtualKeyCode::D, VirtualKeyCode::F, "Release")=> State::Enabled(0.5),
            Item::new_int('c', 'v', VirtualKeyCode::C, VirtualKeyCode::V, "Freq")=> State::Enabled(500.0),
            Item::new_tog('t', 'y', 'u', VirtualKeyCode::T, VirtualKeyCode::Y,VirtualKeyCode::U, "Crunch")=> State::Disabled(0.0),
            Item::new_tog('g', 'h', 'j', VirtualKeyCode::G, VirtualKeyCode::H,VirtualKeyCode::J, "Drive")=> State::Disabled(0.0),
        };

        let shapes = generate_shapes();

        let texts = generate_text();
        let osc_text = osc_text();
        let duty_text = duty_text();

        Controller {
            items,
            audio,
            shapes,
            button_shape: Shape::rect((0, 0), (11, 13), stroke(LIGHT_GRAY)),
            osc_type: OscillatorType::Sine,
            cycle: DutyCycle::Half,
            texts,
            duty_text,
            osc_text,
        }
    }
}

impl Controller {
    pub fn input(&mut self, helper: &WinitInputHelper) {
        for (item, value) in self.items.iter_mut() {
            let mut delta = match item.item_type {
                ItemType::Float => 0.1,
                ItemType::Int => 10.0,
            };
            if helper.held_shift() {
                delta *= 10.0;
            }
            if helper.key_pressed(item.dec_code) {
                if value.num() > delta {
                    *value = value.update(-delta);
                } else {
                    *value = value.replace(0.0);
                }
            }
            if helper.key_pressed(item.inc_code) {
                *value = value.update(delta);
            }
            if let Some(tog) = item.toggle_code {
                if helper.key_pressed(tog) {
                    *value = value.swap();
                }
            }
        }
        if helper.key_pressed(VirtualKeyCode::Key1) {
            self.osc_type = OscillatorType::Sine;
        } else if helper.key_pressed(VirtualKeyCode::Key2) {
            self.osc_type = OscillatorType::Triangle;
        } else if helper.key_pressed(VirtualKeyCode::Key3) {
            self.osc_type = OscillatorType::Saw;
        } else if helper.key_pressed(VirtualKeyCode::Key4) {
            self.osc_type = OscillatorType::Square;
        } else if helper.key_pressed(VirtualKeyCode::Key5) {
            self.osc_type = OscillatorType::Noise;
        }
        if helper.key_pressed(VirtualKeyCode::Key7) {
            self.cycle = DutyCycle::Half;
        } else if helper.key_pressed(VirtualKeyCode::Key8) {
            self.cycle = DutyCycle::Third;
        } else if helper.key_pressed(VirtualKeyCode::Key9) {
            self.cycle = DutyCycle::Quarter;
        } else if helper.key_pressed(VirtualKeyCode::Key0) {
            self.cycle = DutyCycle::Eight;
        }
        if helper.key_pressed(VirtualKeyCode::Space) {
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
            self.audio.play(sample);
        }
    }

    pub fn render(&self, graphics: &mut Graphics<'_>) {
        render_ui(self, graphics)
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
    pub dec_code: VirtualKeyCode,
    pub inc_code: VirtualKeyCode,
    pub toggle_code: Option<VirtualKeyCode>,
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
        dec_code: VirtualKeyCode,
        inc_code: VirtualKeyCode,
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
        tog_code: VirtualKeyCode,
        dec_code: VirtualKeyCode,
        inc_code: VirtualKeyCode,
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
        dec_code: VirtualKeyCode,
        inc_code: VirtualKeyCode,
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
