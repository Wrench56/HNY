extern crate pancurses;

use pancurses::{Window, COLOR_PAIR};

use crate::color;

enum FireworkState {
    Spawn,
    Flying,
    Explosion1,
    Explosion2,
    Explosion3,
    Dead
}


pub struct Firework {
    x_pos: i32,
    y_pos: i32,
    release_height: i32,
    color: u64,
    state: FireworkState,
}

impl Firework {
    pub fn new(x_pos: i32, y_pos: i32, release_height: i32, color: color::ColorPairId) -> Firework {
        Firework{
            x_pos: x_pos,
            y_pos: y_pos,
            release_height: release_height,
            color: color,
            state: FireworkState::Spawn
        }
    }
    pub fn next_cycle(&mut self, stdscr: &Window) -> bool{
        match self.state {
            FireworkState::Spawn => {
                stdscr.attron(COLOR_PAIR(7));
                stdscr.mv(self.y_pos, self.x_pos);
                stdscr.addch('|');
                stdscr.mv(self.y_pos-1, self.x_pos);
                stdscr.addch('.');

                self.y_pos -= 1;

                self.state = FireworkState::Flying;
                true
            }
            FireworkState::Flying => {
                stdscr.attron(COLOR_PAIR(7));
                stdscr.mv(self.y_pos, self.x_pos);
                stdscr.addch('|');
                stdscr.mv(self.y_pos-1, self.x_pos);
                stdscr.addch('.');
                
                self.y_pos -= 1;

                if self.y_pos <= self.release_height {
                    self.state = FireworkState::Explosion1;
                }
                true
            }
            FireworkState::Explosion1 => {
                stdscr.attron(COLOR_PAIR(self.color));
                stdscr.mvprintw(self.y_pos-1, self.x_pos-1, "* *");
                stdscr.mvprintw(self.y_pos, self.x_pos-2, "* # *");
                stdscr.mvprintw(self.y_pos+1, self.x_pos-1, "* *");

                self.state = FireworkState::Explosion2;
                true
            }
            FireworkState::Explosion2 => {
                stdscr.attron(COLOR_PAIR(self.color));
                stdscr.mvprintw(self.y_pos-2, self.x_pos-1, "*''*");
                stdscr.mvprintw(self.y_pos-1, self.x_pos-2, "*_\\/_*");
                stdscr.mvprintw(self.y_pos, self.x_pos-2, "* /\\ *");
                stdscr.mvprintw(self.y_pos+1, self.x_pos-1, "*''*");

                self.state = FireworkState::Explosion3;
                true
            }
            FireworkState::Explosion3 => {
                stdscr.attron(COLOR_PAIR(self.color));
                stdscr.mvprintw(self.y_pos-3, self.x_pos, "..");
                stdscr.mvprintw(self.y_pos-2, self.x_pos-2, "*\\::/*");
                stdscr.mvprintw(self.y_pos-1, self.x_pos-3, "*__\\/__*");
                stdscr.mvprintw(self.y_pos, self.x_pos-3, "*  /\\  *");
                stdscr.mvprintw(self.y_pos+1, self.x_pos-2, "*/::\\*");
                stdscr.mvprintw(self.y_pos+2, self.x_pos, "ˇˇ");

                self.state = FireworkState::Dead;
                true
            }
            FireworkState::Dead => {    
                false
            }
        }
    }
}