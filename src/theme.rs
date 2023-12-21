use pixels_graphics_lib::buffer_graphics_lib::color::*;

pub struct Theme {
    pub active: Color,
    pub background: Color,
    pub disabled: Color,
    pub inactive: Color,
}

pub fn themes() -> Vec<Theme> {
    vec![
        //Dark
        Theme {
            active: WHITE,
            background: BLACK,
            disabled: WHITE.with_brightness(0.2),
            inactive: WHITE.with_brightness(0.5),
        },
        //Gameboy
        Theme {
            active: GB_3,
            background: GB_0,
            disabled: GB_1,
            inactive: GB_2,
        },
        // Hacker
        Theme {
            active: GREEN,
            background: BLACK,
            disabled: GREEN.with_brightness(0.2),
            inactive: GREEN.with_brightness(0.5),
        },
        // New Vegas
        Theme {
            active: ORANGE,
            background: BLACK,
            disabled: ORANGE.with_brightness(0.2),
            inactive: ORANGE.with_brightness(0.5),
        },
        // Light
        Theme {
            active: BLACK,
            background: WHITE,
            disabled: LIGHT_GRAY,
            inactive: DARK_GRAY,
        },
    ]
}
