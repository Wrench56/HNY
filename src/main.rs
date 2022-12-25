extern crate pancurses;

use pancurses::{initscr, endwin, Window};
use std::{env, process::exit};


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: HNY <year>\nArguments:\n - year: provide the new year");
        exit(0)
    }

    let new_year: &u32 = &args[1].parse::<u32>().unwrap();


    let stdscr: Window = initscr();
    stdscr.printw("Hello");
    stdscr.refresh();
    stdscr.getch();
    endwin();
}