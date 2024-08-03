use super::components::{CollisionExtent, PhysicsBody, Stable, Velocity};
use crate::terrain::{Terrain, TerrainCollisionData};
use bevy::prelude::*;

#[derive(Clone, Debug, Event)]
pub struct TerrainCollisionEvent {
    pub entity: Entity,
    pub collision: TerrainCollisionData,
}

// https://github.com/bitshifter/glam-rs/pull/531
fn reflect(incident: Vec2, normal: Vec2) -> Vec2 {
    incident - 2.0 * incident.dot(normal) * normal
}

pub fn handle_terrain_collisions(
    mut ev_collision: EventWriter<TerrainCollisionEvent>,
    mut commands: Commands,
    images: Res<Assets<Image>>,
    time: Res<Time>,
    mut bodies: Query<
        (
            Entity,
            &mut Transform,
            &CollisionExtent,
            &mut Velocity,
            &PhysicsBody,
        ),
        Without<Terrain>,
    >,
    terrains: Query<(&Transform, &Handle<Image>), (With<Sprite>, With<Terrain>)>,
) {
    for (terrain_transform, terrain_image) in &terrains {
        let Some(terrain_image) = images.get(terrain_image) else {
            continue;
        };

        for (entity, mut transform, collision_extent, mut velocity, body) in &mut bodies {
            let delta_seconds = time.delta_seconds();
            let position = transform.translation.xy();
            let potential_position = position + velocity.0 * delta_seconds;
            let velocity_length = (velocity.0 * delta_seconds).length();
            let mut stable = false;

            if let Some(collision) = Terrain::test_collision(
                potential_position,
                collision_extent.0,
                terrain_transform,
                terrain_image,
            ) {
                let response_velocity = potential_position - collision.position;
                let response_direction = response_velocity.normalize();
                velocity.0 = body.restitution * reflect(velocity.0, response_direction);
                stable = true;

                // As this system is the only place where bodies actually touch terrain,
                // we have to send such events instead of checking terrain collisions
                // in systems on demand
                ev_collision.send(TerrainCollisionEvent { entity, collision });
            } else {
                // TODO: move to a separate system
                transform.translation = potential_position.extend(transform.translation.z);
            }

            // TODO: move to a separate system
            const STABILITY_THRESHOLD: f32 = 0.005;
            if velocity_length < STABILITY_THRESHOLD {
                stable = true;
            }

            let mut commands = commands.entity(entity);
            if stable {
                commands.insert(Stable);
            } else {
                commands.remove::<Stable>();
            }
        }
    }
}
