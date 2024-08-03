use super::projectile::ExplodesOnTerrainCollision;
use crate::{game::ExplosionEvent, physics::TerrainCollisionEvent};
use bevy::prelude::*;

pub fn explode_projectiles(
    mut commands: Commands,
    mut ev_collision: EventReader<TerrainCollisionEvent>,
    mut ev_explosion: EventWriter<ExplosionEvent>,
    projectiles: Query<(), With<ExplodesOnTerrainCollision>>,
) {
    const EXPLODE_RADIUS: f32 = 100.0;

    for event in ev_collision.read() {
        let Ok(()) = projectiles.get(event.entity) else {
            continue;
        };
        ev_explosion.send(ExplosionEvent {
            center: event.collision.position,
            radius: EXPLODE_RADIUS,
        });
        commands.entity(event.entity).despawn();
    }
}
