use crate::controller::{Item, ItemType, State};
use crate::Controller;
use buffer_graphics_lib::color::{Color, DARK_GRAY, LIGHT_GRAY, WHITE};
use buffer_graphics_lib::shapes::{stroke, Shape};
use buffer_graphics_lib::text::Text;
use buffer_graphics_lib::text::TextSize::{Large, Small};
use buffer_graphics_lib::Graphics;
use indexmap::{indexmap, IndexMap};
use usfx::{DutyCycle, OscillatorType};

pub fn render_ui(controller: &Controller, graphics: &mut Graphics) {
    for shape in &controller.shapes {
        graphics.draw(shape);
    }
    for text in &controller.texts {
        graphics.draw(text);
    }
    for (osc, text) in &controller.osc_text {
        if osc == &controller.osc_type {
            graphics.draw(&text.with_color(WHITE))
        } else {
            graphics.draw(text)
        }
    }
    for (cycle, text) in &controller.duty_text {
        if cycle == &controller.cycle {
            graphics.draw(&text.with_color(WHITE))
        } else {
            graphics.draw(text)
        }
    }

    let mut y = 50;
    for (item, value) in &controller.items {
        draw_item(graphics, item, value, 4, y, &controller.button_shape);
        y += 16;
    }

    graphics.draw(&Shape::rect((60, 295), (198, 314), stroke(LIGHT_GRAY)));

    draw_button(graphics, '1', 6, 202, LIGHT_GRAY, &controller.button_shape);
    draw_button(graphics, '2', 66, 202, LIGHT_GRAY, &controller.button_shape);
    draw_button(
        graphics,
        '3',
        166,
        202,
        LIGHT_GRAY,
        &controller.button_shape,
    );
    draw_button(graphics, '4', 6, 220, LIGHT_GRAY, &controller.button_shape);
    draw_button(graphics, '5', 86, 220, LIGHT_GRAY, &controller.button_shape);
    draw_button(graphics, '7', 6, 260, LIGHT_GRAY, &controller.button_shape);
    draw_button(graphics, '8', 74, 260, LIGHT_GRAY, &controller.button_shape);
    draw_button(
        graphics,
        '9',
        140,
        260,
        LIGHT_GRAY,
        &controller.button_shape,
    );
    draw_button(
        graphics,
        '0',
        208,
        260,
        LIGHT_GRAY,
        &controller.button_shape,
    );
}

fn draw_item(
    graphics: &mut Graphics,
    item: &Item,
    value: &State,
    x: usize,
    y: usize,
    button_shape: &Shape,
) {
    let (bcolor, vcolor) = if let State::Enabled(_) = value {
        (LIGHT_GRAY, WHITE)
    } else {
        (DARK_GRAY, DARK_GRAY)
    };
    if let Some(tog) = item.toggle {
        draw_button(graphics, tog, x, y, LIGHT_GRAY, button_shape);
    }
    draw_button(graphics, item.dec, x + 16, y, bcolor, button_shape);
    draw_button(graphics, item.inc, x + 34, y, bcolor, button_shape);
    graphics.draw(&Text::new(
        item.name,
        (x as isize + 50, y as isize),
        (WHITE, Large),
    ));
    let text = match item.item_type {
        ItemType::Float => format!("{:0.1}", value.num()),
        ItemType::Int => format!("{}", value.num().round() as usize),
    };
    graphics.draw(&Text::new(
        text,
        (x as isize + 150, y as isize),
        (vcolor, Large),
    ));
}

fn draw_button(
    graphics: &mut Graphics,
    letter: char,
    x: usize,
    y: usize,
    color: Color,
    shape: &Shape,
) {
    graphics.draw_letter((x as isize, y as isize), letter, Large, color);
    graphics.draw_at((x as isize - 2, y as isize - 2), shape);
}

pub fn generate_shapes() -> Vec<Shape> {
    let general_color = LIGHT_GRAY;
    let line_start = 42;
    let x = [8, 24, 40];
    let y = [8, 20, 30];
    let line_end = 56;
    vec![
        Shape::line((x[0], line_start), (x[0], y[0]), general_color),
        Shape::line((x[1], line_start), (x[1], y[1]), general_color),
        Shape::line((x[2], line_start), (x[2], y[2]), general_color),
        Shape::line((x[0], y[0]), (line_end, y[0]), general_color),
        Shape::line((x[1], y[1]), (line_end, y[1]), general_color),
        Shape::line((x[2], y[2]), (line_end, y[2]), general_color),
        Shape::line((80, y[1]), (90, y[1]), general_color),
        Shape::line((80, y[2]), (90, y[2]), general_color),
        Shape::line((90, y[1]), (90, y[2] + 1), general_color),
        Shape::line((90, 25), (96, 25), general_color),
    ]
}

pub fn generate_text() -> Vec<Text> {
    let general_text_color = LIGHT_GRAY;
    vec![
        Text::new("Toggle", (60_isize, 6), (general_text_color, Small)),
        Text::new("Dec", (60_isize, 18), (general_text_color, Small)),
        Text::new("Inc", (60_isize, 28), (general_text_color, Small)),
        Text::new(
            "Shift to inc/dec faster",
            (100_isize, 22),
            (general_text_color, Small),
        ),
        Text::new("Oscillator", (4_isize, 186), (general_text_color, Large)),
        Text::new("Duty Cycle", (4_isize, 244), (general_text_color, Large)),
        Text::new(
            "Space to play",
            (65_isize, 300),
            (general_text_color, Large),
        ),
    ]
}

pub fn osc_text() -> IndexMap<OscillatorType, Text> {
    let general_text_color = LIGHT_GRAY;
    indexmap![
        OscillatorType::Sine => Text::new("Sine", (20_isize, 202), (general_text_color, Large)),
        OscillatorType::Triangle => Text::new("Triangle", (80_isize, 202), (general_text_color, Large)),
        OscillatorType::Saw => Text::new("Saw", (180_isize, 202), (general_text_color, Large)),
        OscillatorType::Square => Text::new("Square", (20_isize, 220), (general_text_color, Large)),
        OscillatorType::Noise => Text::new("Noise", (100_isize, 220), (general_text_color, Large)),
    ]
}

pub fn duty_text() -> IndexMap<DutyCycle, Text> {
    let general_text_color = LIGHT_GRAY;
    indexmap![
        DutyCycle::Half => Text::new("1/2", (20_isize, 260), (general_text_color, Large)),
        DutyCycle::Third => Text::new("1/3", (92_isize, 260), (general_text_color, Large)),
        DutyCycle::Quarter => Text::new("1/4", (156_isize, 260), (general_text_color, Large)),
        DutyCycle::Eight => Text::new("1/8", (224_isize, 260), (general_text_color, Large)),
    ]
}
