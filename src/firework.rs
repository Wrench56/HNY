extern crate pancurses;

use pancurses::Window;

enum FireworkState {
    Spawn,
    Flying,
    Explosion
}


pub struct Firework {
    x_pos: i32,
    y_pos: i32,
    release_height: i32,
    color: i16,
    state: FireworkState,
}

impl Firework {
    pub fn new(x_pos: i32, y_pos: i32, release_height: i32, color: i16) -> Firework {
        Firework{
            x_pos: x_pos,
            y_pos: y_pos,
            release_height: release_height,
            color: color,
            state: FireworkState::Spawn
        }
    }
    pub fn next_cycle(&mut self, _stdscr: &Window) {
        match self.state {
            FireworkState::Spawn => {
                self.state = FireworkState::Flying;
            }
            FireworkState::Flying => {}
            FireworkState::Explosion => {}
        }
    }
}