use super::components::{AimingAngle, Weapon};
use bevy::prelude::*;

pub fn apply_aiming_angle(mut query: Query<(&mut Transform, &AimingAngle), With<Weapon>>) {
    for (mut transform, aiming_angle) in &mut query {
        transform.rotation = Quat::from_rotation_z(aiming_angle.0);
    }
}
