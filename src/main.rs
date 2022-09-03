#![windows_subsystem = "windows"]

mod audio;

use crate::audio::Audio;
use anyhow::Result;
use buffer_graphics_lib::color::{Color, BLACK, DARK_GRAY, LIGHT_GRAY, WHITE};
use buffer_graphics_lib::drawing::DrawingMethods;
use buffer_graphics_lib::text::TextPos;
use buffer_graphics_lib::text::TextSize::{Normal, Small};
use buffer_graphics_lib::Graphics;
use indexmap::{indexmap, IndexMap};
use pixels_graphics_lib::prefs::WindowPreferences;
use pixels_graphics_lib::{setup, WindowScaling};
use std::thread::sleep;
use std::time::Duration;
use usfx::{DutyCycle, OscillatorType, Sample};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;

const WIDTH: usize = 260;
const HEIGHT: usize = 320;

fn main() -> Result<()> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let (mut window, mut pixels) = setup(
        (WIDTH, HEIGHT),
        WindowScaling::Auto,
        "USFX Test",
        &event_loop,
    )?;
    let mut prefs = WindowPreferences::new("app", "raybritton", "usfx_tester")?;
    prefs.load()?;
    prefs.restore(&mut window);

    let mut audio = Audio::new();
    audio.run();
    let mut basic = Basic::new(audio);

    event_loop.run(move |event, _, control_flow| {
        if let Event::LoopDestroyed = event {
            prefs.store(&window);
            //can't return from here so just print out error
            let _ = prefs
                .save()
                .map_err(|err| eprintln!("Unable to save prefs: {:?}", err));
        }

        if let Event::RedrawRequested(_) = event {
            let mut graphics = Graphics::new(pixels.get_frame(), WIDTH, HEIGHT).unwrap();
            graphics.clear(BLACK);
            basic.render(&mut graphics);
            if pixels
                .render()
                .map_err(|e| eprintln!("pixels.render() failed: {:?}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        basic.update();

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            basic.input(&input);

            window.request_redraw();
        }

        sleep(Duration::from_millis(1));
    });
}

struct Basic {
    items: IndexMap<Item, State>,
    audio: Audio,
    osc_type: OscillatorType,
    cycle: DutyCycle,
}

impl Basic {
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

        Basic {
            items,
            audio,
            osc_type: OscillatorType::Sine,
            cycle: DutyCycle::Half,
        }
    }
}

impl Basic {
    fn update(&mut self) {}

    fn input(&mut self, helper: &WinitInputHelper) {
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

    fn render(&self, graphics: &mut Graphics<'_>) {
        let mut y = 50;
        for (item, value) in &self.items {
            Basic::draw_item(graphics, item, value, 4, y);
            y += 16;
        }
        let line_bottom = 42;
        let text_x = 50;
        graphics.draw_line(8, line_bottom, 8, 8, LIGHT_GRAY);
        graphics.draw_line(8, 8, text_x - 4, 8, LIGHT_GRAY);
        graphics.draw_text("Toggle", None, TextPos::Px(text_x, 6), Small, LIGHT_GRAY);
        graphics.draw_line(22, line_bottom, 22, 18, LIGHT_GRAY);
        graphics.draw_line(22, 18, text_x - 4, 18, LIGHT_GRAY);
        graphics.draw_text("Dec", None, TextPos::Px(text_x, 16), Small, LIGHT_GRAY);
        graphics.draw_line(42, line_bottom, 42, 30, LIGHT_GRAY);
        graphics.draw_line(42, 30, text_x - 4, 30, LIGHT_GRAY);
        graphics.draw_text("Inc", None, TextPos::Px(text_x, 28), Small, LIGHT_GRAY);
        graphics.draw_line(70, 30, 80, 30, LIGHT_GRAY);
        graphics.draw_line(70, 18, 80, 18, LIGHT_GRAY);
        graphics.draw_line(80, 18, 80, 30, LIGHT_GRAY);
        graphics.draw_line(80, 24, 90, 24, LIGHT_GRAY);
        graphics.draw_text(
            "Shift to inc/dec faster",
            None,
            TextPos::Px(100, 22),
            Small,
            LIGHT_GRAY,
        );

        graphics.draw_text(
            "SPACE to play",
            None,
            TextPos::Px(65, 300),
            Normal,
            LIGHT_GRAY,
        );
        graphics.draw_frame(60, 295, 198, 314, LIGHT_GRAY);

        graphics.draw_text("OSCILLATOR", None, TextPos::Px(4, 186), Normal, LIGHT_GRAY);
        Basic::draw_button(graphics, '1', 6, 202, LIGHT_GRAY);
        graphics.draw_text(
            "SINE",
            None,
            TextPos::Px(20, 202),
            Normal,
            if self.osc_type == OscillatorType::Sine {
                WHITE
            } else {
                LIGHT_GRAY
            },
        );
        Basic::draw_button(graphics, '2', 66, 202, LIGHT_GRAY);
        graphics.draw_text(
            "TRIANGLE",
            None,
            TextPos::Px(80, 202),
            Normal,
            if self.osc_type == OscillatorType::Triangle {
                WHITE
            } else {
                LIGHT_GRAY
            },
        );
        Basic::draw_button(graphics, '3', 166, 202, LIGHT_GRAY);
        graphics.draw_text(
            "SAW",
            None,
            TextPos::Px(180, 202),
            Normal,
            if self.osc_type == OscillatorType::Saw {
                WHITE
            } else {
                LIGHT_GRAY
            },
        );
        Basic::draw_button(graphics, '4', 6, 220, LIGHT_GRAY);
        graphics.draw_text(
            "SQUARE",
            None,
            TextPos::Px(20, 220),
            Normal,
            if self.osc_type == OscillatorType::Square {
                WHITE
            } else {
                LIGHT_GRAY
            },
        );
        Basic::draw_button(graphics, '5', 86, 220, LIGHT_GRAY);
        graphics.draw_text(
            "NOISE",
            None,
            TextPos::Px(100, 220),
            Normal,
            if self.osc_type == OscillatorType::Noise {
                WHITE
            } else {
                LIGHT_GRAY
            },
        );

        graphics.draw_text("DUTY CYCLE", None, TextPos::Px(4, 244), Normal, LIGHT_GRAY);
        Basic::draw_button(graphics, '7', 6, 260, LIGHT_GRAY);
        graphics.draw_text(
            "1/2",
            None,
            TextPos::Px(20, 260),
            Normal,
            if self.cycle == DutyCycle::Half {
                WHITE
            } else {
                LIGHT_GRAY
            },
        );
        Basic::draw_button(graphics, '8', 74, 260, LIGHT_GRAY);
        graphics.draw_text(
            "1/3",
            None,
            TextPos::Px(92, 260),
            Normal,
            if self.cycle == DutyCycle::Third {
                WHITE
            } else {
                LIGHT_GRAY
            },
        );
        Basic::draw_button(graphics, '9', 140, 260, LIGHT_GRAY);
        graphics.draw_text(
            "1/4",
            None,
            TextPos::Px(156, 260),
            Normal,
            if self.cycle == DutyCycle::Quarter {
                WHITE
            } else {
                LIGHT_GRAY
            },
        );
        Basic::draw_button(graphics, '0', 208, 260, LIGHT_GRAY);
        graphics.draw_text(
            "1/8",
            None,
            TextPos::Px(224, 260),
            Normal,
            if self.cycle == DutyCycle::Eight {
                WHITE
            } else {
                LIGHT_GRAY
            },
        );
    }

    fn draw_item(graphics: &mut Graphics, item: &Item, value: &State, x: usize, y: usize) {
        let (bcolor, vcolor) = if let State::Enabled(_) = value {
            (LIGHT_GRAY, WHITE)
        } else {
            (DARK_GRAY, DARK_GRAY)
        };
        if let Some(tog) = item.toggle {
            Basic::draw_button(graphics, tog, x, y, LIGHT_GRAY);
        }
        Basic::draw_button(graphics, item.dec, x + 16, y, bcolor);
        Basic::draw_button(graphics, item.inc, x + 34, y, bcolor);
        graphics.draw_text(
            item.name,
            Some(14),
            TextPos::usize_px(x + 50, y),
            Normal,
            WHITE,
        );
        let text = match item.item_type {
            ItemType::Float => format!("{:0.1}", value.num()),
            ItemType::Int => format!("{}", value.num().round() as usize),
        };
        graphics.draw_text(&text, None, TextPos::usize_px(x + 150, y), Normal, vcolor);
    }

    fn draw_button(graphics: &mut Graphics, letter: char, x: usize, y: usize, color: Color) {
        graphics.draw_text(
            &letter.to_string(),
            None,
            TextPos::usize_px(x, y),
            Normal,
            color,
        );
        graphics.draw_frame(x - 2, y - 2, x + 9, y + 11, color);
    }
}

enum State {
    Enabled(f32),
    Disabled(f32),
}

impl State {
    fn num(&self) -> f32 {
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
struct Item {
    dec: char,
    inc: char,
    toggle: Option<char>,
    dec_code: VirtualKeyCode,
    inc_code: VirtualKeyCode,
    toggle_code: Option<VirtualKeyCode>,
    name: &'static str,
    item_type: ItemType,
}

#[derive(Hash, Debug, Eq, PartialEq)]
enum ItemType {
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
