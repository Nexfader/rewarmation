use super::components::PhysicsBody;
use bevy::prelude::*;

pub fn destroy_out_of_bounds_objects(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<PhysicsBody>>,
) {
    for (entity, transform) in &query {
        if transform.translation.y < 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
