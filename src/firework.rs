enum FireworkState {
    Spawn,
    Flying,
    Explosion
}


struct Firework {
    state: FireworkState,
}

impl Firework {
    fn next_cycle(&self) {
        match self.state {
            FireworkState::Spawn =>{}
            FireworkState::Flying => {}
            FireworkState::Explosion => {}
        }
    }
}