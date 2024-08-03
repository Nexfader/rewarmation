use super::components::{PhysicsBody, Velocity};
use bevy::prelude::*;

pub fn apply_gravity(mut query: Query<(&mut Velocity, &PhysicsBody)>) {
    const GRAVITY: Vec2 = Vec2::new(0.0, -15.0);

    for (mut velocity, body) in &mut query {
        velocity.add_force(body.mass * GRAVITY);
    }
}
