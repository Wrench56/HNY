extern crate pancurses;
extern crate rand;

use pancurses::{initscr, endwin, Window};
use std::{env, process::exit, thread, time};
use rand::Rng;

use unicode_segmentation::UnicodeSegmentation;
use console::Term;

mod art;
mod color;

mod firework;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: HNY <year>\nArguments:\n - year: provide the new year");
        exit(0)
    }

    let new_year: &u32 = &args[1].parse::<u32>().unwrap();

    // Initialize ncurses window
    let stdscr: Window = initscr();

    // Initialize ncurses colors
    color::init_colors();

    // Create exit thread
    thread::spawn(|| {
        exit_thread();
    });

    prev_to_new(&stdscr, &new_year);
    firework_loop(&stdscr);

}

fn prev_to_new(stdscr: &Window, _new_year: &u32) {
    const PADDING: i32 = 3;

    let x: i32 = stdscr.get_max_x();
    let y: i32 = stdscr.get_max_y();

    let prev_year: Vec<String> = art::str_to_art(&(_new_year-1).to_string());
    let curr_year: Vec<String> = art::str_to_art(&_new_year.to_string());

    let year_x_pos: i32 = (((x-(prev_year[0].graphemes(true).count() as i32))/2) as f32).floor() as i32;

    let mut year_y_pos: i32 = 0;
    for i in 0..(y-(prev_year.len() as i32)-PADDING) {
        stdscr.erase();
        year_y_pos = i;
        for line in &prev_year {
            stdscr.mv(year_y_pos+PADDING, year_x_pos);
            stdscr.addstr(line);
            year_y_pos += 1;
        }
        stdscr.refresh();       
        thread::sleep(time::Duration::from_millis(30));
    }
    stdscr.erase();
    stdscr.refresh();  
    thread::sleep(time::Duration::from_secs(1));

    for i in (0..y-(prev_year.len() as i32)-PADDING).rev() {
        stdscr.erase();
        year_y_pos = i;
        for line in &curr_year {
            stdscr.mv(year_y_pos+PADDING, year_x_pos);
            stdscr.addstr(line);
            year_y_pos += 1;
        }
        stdscr.refresh();       
        thread::sleep(time::Duration::from_millis(40));
    }
}

fn firework_loop(stdscr: &Window) {
    let x: i32 = stdscr.get_max_x();
    let y: i32 = stdscr.get_max_y();

    let mut rng = rand::thread_rng();

    let mut fireworks: Vec<firework::Firework> = Vec::new();
    loop {
        for _ in 0..rng.gen_range(0..4) {
            fireworks.push(firework::Firework::new(
                rng.gen_range(4..=(x-4)),
                y - 2,
                rng.gen_range(4..((((y/3) as f32)*2 as f32) as f32).floor() as i32),
                rng.gen_range(1..7) as u64,
            ));
        }
        let mut item_index: i16 = fireworks.len() as i16 - 1 ; // i8 might be sufficient
        while item_index > 0 {
            if fireworks[item_index as usize].next_cycle(stdscr) == false {
                fireworks.remove(item_index as usize);
            }
            item_index -= 1;
        }
        stdscr.refresh();
        thread::sleep(time::Duration::from_millis(128));
        stdscr.erase();
    }
}


fn exit_thread() {
    let stdout = Term::buffered_stdout();

    loop {
        if let Ok(character) = stdout.read_char() {
            match character {
                'q' => break,
                _ => {}
            }
        }
    }
    endwin();
    exit(0)
}