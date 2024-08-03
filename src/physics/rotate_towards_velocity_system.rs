use super::components::{RotatesTowardsDirection, Velocity};
use bevy::prelude::*;

pub fn rotate_objects_towards_velocity(
    mut query: Query<(&mut Transform, &Velocity), With<RotatesTowardsDirection>>,
) {
    for (mut transform, velocity) in &mut query {
        transform.rotation = Quat::from_rotation_z(velocity.0.y.atan2(velocity.0.x));
    }
}
