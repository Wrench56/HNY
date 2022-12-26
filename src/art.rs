extern crate cfonts;

use cfonts::{render, Options, Fonts};

pub fn str_to_art(string: &String) -> Vec<String>{
    render(Options {
		text: String::from(string),
		font: Fonts::FontHuge,
		..Options::default()
    }).text.lines().filter(|x| *x != "").map(str::to_string).collect()
}