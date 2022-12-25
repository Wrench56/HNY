extern crate pancurses;

use pancurses::{initscr, endwin, Window};
use std::{env, process::exit};

mod art;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: HNY <year>\nArguments:\n - year: provide the new year");
        exit(0)
    }

    let new_year: &u32 = &args[1].parse::<u32>().unwrap();


    let stdscr: Window = initscr();
    stdscr.printw("Hello");
    prev_to_new(&stdscr, &new_year)
    stdscr.refresh();
    stdscr.getch();
    endwin();
}

fn prev_to_new(_stdscr: &Window, _new_year: &u32) {
    art::str_to_art(&(_new_year-1).to_string());
}