extern crate cfonts;
extern crate pancurses;

use pancurses::Window;
use cfonts::{render, Options, Fonts};

pub fn str_to_art(string: &String) -> Vec<String>{
    render(Options {
		text: String::from(string),
		font: Fonts::FontHuge,
		..Options::default()
    }).text.lines().filter(|x| *x != "").map(str::to_string).collect()
}

pub fn draw_ascii_text(stdscr: &Window, ascii_text: &Vec<String>, y: &mut i32, x: &i32) {
	for line in ascii_text {
		stdscr.mv(*y, *x);
		stdscr.addstr(line);
		*y += 1;
	}
}