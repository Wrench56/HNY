extern crate pancurses;

use pancurses::{start_color, use_default_colors, init_pair, OK};
use pancurses::{COLOR_BLACK, COLOR_BLUE, COLOR_GREEN, COLOR_CYAN, COLOR_RED, COLOR_MAGENTA, COLOR_YELLOW, COLOR_WHITE};

const COLOR_TABLE: [i16; 8] = [
    COLOR_BLACK,
    COLOR_BLUE,
    COLOR_GREEN,
    COLOR_CYAN,
    COLOR_RED,
    COLOR_MAGENTA,
    COLOR_YELLOW,
    COLOR_WHITE,
];

pub fn init_colors() {
    let mut bg = COLOR_BLACK;

    start_color();
    if use_default_colors() == OK {
        bg = -1;
    }

    for (i, color) in COLOR_TABLE.iter().enumerate() {
        init_pair(i as i16, *color, COLOR_BLACK);
    }

}

