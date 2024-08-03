mod components;
mod gravity_system;
mod out_of_bounds_destroy_system;
mod rotate_towards_velocity_system;
mod terrain_collision_system;

pub use components::{
    CollisionExtent, PhysicsBody, PhysicsBodyBundle, RotatesTowardsDirection, Stable, Velocity,
};
pub use terrain_collision_system::TerrainCollisionEvent;

use crate::game::{GamePlayState, PlayingState};
use bevy::prelude::*;
use gravity_system::apply_gravity;
use out_of_bounds_destroy_system::destroy_out_of_bounds_objects;
use rotate_towards_velocity_system::rotate_objects_towards_velocity;
use terrain_collision_system::handle_terrain_collisions;

pub struct PhysicsPlugin;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PhysicsSystemSet;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                apply_gravity,
                handle_terrain_collisions,
                destroy_out_of_bounds_objects,
                rotate_objects_towards_velocity,
            )
                .in_set(PhysicsSystemSet)
                .run_if(in_state(GamePlayState::InGame))
                .run_if(in_state(PlayingState::Playing)),
        )
        .add_event::<TerrainCollisionEvent>();
    }
}
