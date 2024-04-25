use crate::controller::*;
use crate::settings::SoundSave;
use crate::theme::Theme;
use crate::waveform::Waveform;
use indexmap::{indexmap, IndexMap};
use pixels_graphics_lib::buffer_graphics_lib::clipping::Clip;
use pixels_graphics_lib::buffer_graphics_lib::prelude::TextPos::Px;
use pixels_graphics_lib::buffer_graphics_lib::prelude::*;
use pixels_graphics_lib::prelude::PixelFont::{Limited3x5, Standard4x5, Standard6x7, Standard8x10};
use usfx::*;

pub fn render_ui(
    controller: &Controller,
    graphics: &mut Graphics,
    theme: &Theme,
    active_theme: usize,
    waveform: &Waveform,
    saves: &[Option<SoundSave>],
) {
    controller.shapes.render(graphics);
    for text in &controller.texts {
        graphics.draw(text);
    }
    for (osc, text) in &controller.osc_text {
        if osc == &controller.osc_type {
            graphics.draw(&text.with_color(theme.active))
        } else {
            graphics.draw(text)
        }
    }
    for (cycle, text) in &controller.duty_text {
        if cycle == &controller.cycle {
            graphics.draw(&text.with_color(theme.active))
        } else {
            graphics.draw(text)
        }
    }

    let mut y = 50;
    for (item, value) in &controller.items {
        draw_item(graphics, theme, item, value, 4, y, &controller.button_shape);
        y += 16;
    }

    graphics.draw_rect(Rect::new((60, 346), (198, 365)), stroke(theme.inactive));

    draw_button(
        graphics,
        'I',
        6,
        198,
        theme.inactive,
        &controller.button_shape,
    );
    draw_button(
        graphics,
        'O',
        66,
        198,
        theme.inactive,
        &controller.button_shape,
    );
    draw_button(
        graphics,
        'P',
        166,
        198,
        theme.inactive,
        &controller.button_shape,
    );
    draw_button(
        graphics,
        'K',
        6,
        216,
        theme.inactive,
        &controller.button_shape,
    );
    draw_button(
        graphics,
        'L',
        86,
        216,
        theme.inactive,
        &controller.button_shape,
    );
    draw_button(
        graphics,
        'B',
        6,
        252,
        theme.inactive,
        &controller.button_shape,
    );
    draw_button(
        graphics,
        'N',
        74,
        252,
        theme.inactive,
        &controller.button_shape,
    );
    draw_button(
        graphics,
        'M',
        6,
        272,
        theme.inactive,
        &controller.button_shape,
    );
    draw_button(
        graphics,
        ',',
        74,
        272,
        theme.inactive,
        &controller.button_shape,
    );
    draw_theme(graphics, theme, active_theme);

    draw_waveform(graphics, theme, waveform);
    draw_duration(graphics, theme, waveform);

    saves.iter().take(10).enumerate().for_each(|(i, save)| {
        if let Some(save) = save {
            graphics.with_translate(coord!(225, 40 + (i * 22)), |g| {
                draw_save(g, theme, i + 1, &save.name, &save.formatted_when());
            });
        }
    });
}

fn draw_waveform(graphics: &mut Graphics, theme: &Theme, waveform: &Waveform) {
    graphics.with_translate(coord!(3, 297), |graphics| {
        graphics.set_clip(Clip::new(337, 48));
        waveform.render_line(graphics, theme.inactive);
        graphics.clip_mut().set_all_valid();
    });
}

fn draw_duration(graphics: &mut Graphics, theme: &Theme, waveform: &Waveform) {
    graphics.draw_text(
        &format!("{:.1}", waveform.duration),
        Px(333, 296),
        (theme.active, Standard6x7, Positioning::RightBottom),
    );
    graphics.draw_text(
        "S",
        Px(338, 296),
        (theme.inactive, Standard4x5, Positioning::RightBottom),
    );
}

fn draw_item(
    graphics: &mut Graphics,
    theme: &Theme,
    item: &Item,
    value: &State,
    x: usize,
    y: usize,
    button_shape: &Drawable<Rect>,
) {
    let (bcolor, vcolor) = if let State::Enabled(_) = value {
        (theme.inactive, theme.active)
    } else {
        (theme.disabled, theme.disabled)
    };
    if let Some(tog) = item.toggle {
        draw_button(graphics, tog, x, y, theme.inactive, button_shape);
    }
    draw_button(graphics, item.dec, x + 16, y, bcolor, button_shape);
    draw_button(graphics, item.inc, x + 34, y, bcolor, button_shape);
    graphics.draw(&Text::new(
        item.name,
        Px(x as isize + 50, y as isize),
        (theme.active, Standard8x10),
    ));
    let text = match item.item_type {
        ItemType::Float => format!("{:0.2}", value.num()),
        ItemType::Int => format!("{}", value.num().round() as usize),
    };
    graphics.draw(&Text::new(
        &text,
        Px(x as isize + 150, y as isize),
        (vcolor, Standard8x10),
    ));
}

fn draw_button(
    graphics: &mut Graphics,
    letter: char,
    x: usize,
    y: usize,
    color: Color,
    shape: &Drawable<Rect>,
) {
    graphics.draw_letter((x as isize, y as isize), letter, Standard8x10, color);
    shape
        .with_move((x as isize - 2, y as isize - 3))
        .render(graphics);
}

pub fn generate_shapes(theme: &Theme) -> ShapeCollection {
    let general_color = theme.inactive;
    let line_start = 42;
    let x = [8, 24, 40];
    let y = [8, 20, 30];
    let line_end = 56;
    let mut collection = ShapeCollection::default();
    InsertShape::insert_above(
        &mut collection,
        Line::new((x[0], line_start), (x[0], y[0])),
        stroke(general_color),
    );
    InsertShape::insert_above(
        &mut collection,
        Line::new((x[1], line_start), (x[1], y[1])),
        stroke(general_color),
    );
    InsertShape::insert_above(
        &mut collection,
        Line::new((x[2], line_start), (x[2], y[2])),
        stroke(general_color),
    );
    InsertShape::insert_above(
        &mut collection,
        Line::new((x[0], y[0]), (line_end, y[0])),
        stroke(general_color),
    );
    InsertShape::insert_above(
        &mut collection,
        Line::new((x[1], y[1]), (line_end, y[1])),
        stroke(general_color),
    );
    InsertShape::insert_above(
        &mut collection,
        Line::new((x[2], y[2]), (line_end, y[2])),
        stroke(general_color),
    );
    InsertShape::insert_above(
        &mut collection,
        Line::new((75, y[1]), (85, y[1])),
        stroke(general_color),
    );
    InsertShape::insert_above(
        &mut collection,
        Line::new((75, y[2]), (85, y[2])),
        stroke(general_color),
    );
    InsertShape::insert_above(
        &mut collection,
        Line::new((85, y[1]), (85, y[2])),
        stroke(general_color),
    );
    InsertShape::insert_above(
        &mut collection,
        Line::new((85, 25), (91, 25)),
        stroke(general_color),
    );
    //waveform box
    InsertShape::insert_above(
        &mut collection,
        Rect::new((2, 288), (36, 296)),
        stroke(general_color),
    );
    InsertShape::insert_above(
        &mut collection,
        Rect::new((2, 296), (338, 340)),
        stroke(general_color),
    );
    InsertShape::insert_above(
        &mut collection,
        Rect::new((260, 286), (338, 296)),
        stroke(general_color),
    );
    InsertShape::insert_above(
        &mut collection,
        Rect::new((223, 2), (280, 18)),
        stroke(general_color),
    );
    InsertShape::insert_above(
        &mut collection,
        Rect::new((223, 18), (338, 260)),
        stroke(general_color),
    );
    InsertShape::insert_above(
        &mut collection,
        Line::new((223, 38), (338, 38)),
        stroke(general_color),
    );
    collection
}

pub fn generate_text(theme: &Theme) -> Vec<Text> {
    let general_text_color = theme.inactive;
    vec![
        Text::new("TOGGLE", Px(60, 6), (general_text_color, Limited3x5)),
        Text::new("DEC", Px(60, 18), (general_text_color, Limited3x5)),
        Text::new("INC", Px(60, 28), (general_text_color, Limited3x5)),
        Text::new("HOLD", Px(95,23), (general_text_color, Limited3x5)),
        Text::new("SHIFT FOR BIGGER", Px(98,29), (general_text_color, Limited3x5)),
        Text::new("CONTROL FOR SMALLER", Px(98,35), (general_text_color, Limited3x5)),
        Text::new("Oscillator", Px(4, 182), (general_text_color, Standard8x10)),
        Text::new("Duty Cycle", Px(4, 236), (general_text_color, Standard8x10)),
        Text::new("(Square only)", Px(110, 240), (general_text_color, Standard4x5)),
        Text::new("Space to play", Px(65, 351), (general_text_color, Standard8x10)),
        Text::new("Saved", Px(225, 6), (general_text_color, Standard8x10)),
        Text::new("WAVEFORM", Px(4, 290), (general_text_color, Limited3x5)),
        Text::new("1-9 TO SAVE", Px(225, 20), (general_text_color, Limited3x5)),
        Text::new("+SHIFT TO LOAD", Px(225, 26), (general_text_color, Limited3x5)),
        Text::new("+CTRL TO DELETE", Px(225, 32), (general_text_color, Limited3x5)),
        Text::new("DURATION", Px(262, 290), (general_text_color, Limited3x5)),
    ]
}

pub fn osc_text(theme: &Theme) -> IndexMap<OscillatorType, Text> {
    let general_text_color = theme.inactive;
    indexmap![
        OscillatorType::Sine => Text::new("Sine", Px(20, 198), (general_text_color, Standard8x10)),
        OscillatorType::Triangle => Text::new("Triangle",Px (80, 198), (general_text_color, Standard8x10)),
        OscillatorType::Saw => Text::new("Saw", Px(180, 198), (general_text_color, Standard8x10)),
        OscillatorType::Square => Text::new("Square", Px(20, 216), (general_text_color, Standard8x10)),
        OscillatorType::Noise => Text::new("Noise", Px(100, 216), (general_text_color, Standard8x10)),
    ]
}

pub fn duty_text(theme: &Theme) -> IndexMap<DutyCycle, Text> {
    let general_text_color = theme.inactive;
    indexmap![
        DutyCycle::Half => Text::new("1/2", Px(20, 252), (general_text_color, Standard8x10)),
        DutyCycle::Third => Text::new("1/3", Px(92, 252), (general_text_color, Standard8x10)),
        DutyCycle::Quarter => Text::new("1/4",Px (20, 272), (general_text_color, Standard8x10)),
        DutyCycle::Eight => Text::new("1/8",Px (92, 272), (general_text_color, Standard8x10)),
    ]
}

pub fn draw_theme(graphics: &mut Graphics, theme: &Theme, active: usize) {
    graphics.draw_text("[ARROWS] THEME", Px(267, 346), (theme.inactive, Standard4x5));
    let width = 60;
    let count = 3;
    let padding = 6;
    let offset = (width - padding * 2) / count;
    for i in 0..=4 {
        draw_theme_box(graphics, theme, i, active == i, offset);
    }
}

pub fn draw_theme_box(
    graphics: &mut Graphics,
    theme: &Theme,
    which: usize,
    filled: bool,
    offset: usize,
) {
    let color = if filled {
        fill(theme.active)
    } else {
        stroke(theme.inactive)
    };
    graphics.draw_rect(
        Rect::new_with_size((267 + offset * which, 358), 6, 6),
        color,
    );
}

pub fn draw_save(graphics: &mut Graphics, theme: &Theme, mut idx: usize, name: &str, when: &str) {
    graphics.draw_rect(Rect::new_with_size((0, 0), 13, 20), stroke(theme.active));
    graphics.draw_rect(Rect::new_with_size((0, 0), 111, 20), stroke(theme.active));
    if idx == 10 {
        //fix last slot number rendering
        idx = 0;
    }
    graphics.draw_text(&format!("{idx}"), Px(3, 5), (theme.active, Standard8x10));
    let (part1, part2) = name.split_at(16);
    graphics.draw_text(part1, Px(16, 2), (theme.inactive, Standard4x5));
    graphics.draw_text(part2.trim(), Px(16, 8), (theme.inactive, Standard4x5));
    graphics.draw_text(when, Px(16, 14), (theme.inactive, Standard4x5));
}
