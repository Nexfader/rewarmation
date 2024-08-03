use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Weapon {
    pub chargeable: bool,
}

#[derive(Component, Copy, Clone, Debug, Default)]
pub struct AimingAngle(pub f32);

impl AimingAngle {
    pub fn bounds(is_flipped: bool) -> (f32, f32) {
        let bounds: (f32, f32) = if is_flipped {
            (91.0, 269.0)
        } else {
            (-89.0, 89.0)
        };
        (bounds.0.to_radians(), bounds.1.to_radians())
    }

    pub fn is_in_bounds(self, is_flipped: bool) -> bool {
        let bounds = Self::bounds(is_flipped);
        (bounds.0..=bounds.1).contains(&self.0)
    }

    pub fn clamp_to_bounds(&mut self, is_flipped: bool) {
        let bounds = Self::bounds(is_flipped);
        self.0 = self.0.clamp(bounds.0, bounds.1);
    }

    pub fn flip(&mut self) {
        self.0 = 180f32.to_radians() - self.0;
    }

    pub fn direction(self) -> Vec2 {
        Vec2::new(self.0.cos(), self.0.sin())
    }
}

#[derive(Component, Copy, Clone, Debug, Default)]
pub struct ShootingForce(pub f32);

impl ShootingForce {
    pub fn is_ready(self) -> bool {
        self.0 >= 1.0
    }

    pub fn is_empty(self) -> bool {
        self.0 <= 0.0
    }
}

#[derive(Bundle, Default)]
pub struct WeaponBundle {
    pub weapon: Weapon,
    pub sprite: SpriteBundle,
    pub aiming_angle: AimingAngle,
    pub shooting_force: ShootingForce,
}
