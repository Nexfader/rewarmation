use bevy::prelude::*;

#[derive(Clone, Copy, Component, Debug, PartialEq, PartialOrd)]
pub struct Health(pub f32);

impl Default for Health {
    fn default() -> Self {
        Self::FULL
    }
}

impl Health {
    pub const FULL: Health = Health(100.0);
    pub const DEAD: Health = Health(0.0);

    pub fn is_alive(self) -> bool {
        self.0 > 0.0
    }

    pub fn is_dead(self) -> bool {
        !self.is_alive()
    }
}
