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
    fn next_cycle(&mut self) {
        match self.state {
            FireworkState::Spawn => {
                self.state = FireworkState::Flying;
            }
            FireworkState::Flying => {}
            FireworkState::Explosion => {}
        }
    }
}