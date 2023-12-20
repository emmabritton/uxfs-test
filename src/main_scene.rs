use crate::*;
use pixels_graphics_lib::buffer_graphics_lib::prelude::{Coord, BLACK};
use pixels_graphics_lib::buffer_graphics_lib::Graphics;
use pixels_graphics_lib::prelude::SceneUpdateResult::Nothing;
use pixels_graphics_lib::prelude::*;
use pixels_graphics_lib::scenes::SceneUpdateResult::Pop;

pub struct MainScene {
    controller: Controller,
    result: SceneUpdateResult<SR, SN>,
    next_input: f64,
}

impl MainScene {
    pub fn new() -> MainScene {
        let mut audio = Audio::new();
        audio.run();
        MainScene {
            controller: Controller::new(audio),
            result: Nothing,
            next_input: 0.0,
        }
    }
}

impl Scene<SR, SN> for MainScene {
    fn render(&self, graphics: &mut Graphics, _: Coord) {
        graphics.clear(BLACK);
        self.controller.render(graphics);
    }

    fn on_key_down(&mut self, key: KeyCode, _: Coord, held_keys: &Vec<&KeyCode>) {
        if self.next_input <= 0.0 {
            self.controller.key_pressed(
                key,
                held_keys.contains(&&KeyCode::ShiftLeft)
                    || held_keys.contains(&&KeyCode::ShiftRight),
            );
            self.next_input = 0.1;
        }
        if key == KeyCode::Escape {
            self.result = Pop(None);
        }
    }

    fn update(
        &mut self,
        timing: &Timing,
        _: Coord,
        _: &Vec<&KeyCode>,
    ) -> SceneUpdateResult<SR, SN> {
        self.next_input -= timing.fixed_time_step;
        self.result.clone()
    }

    fn resuming(&mut self, _: Option<SR>) {}
}
