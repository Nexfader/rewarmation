use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct PhysicsBodyBundle {
    pub body: PhysicsBody,
    pub velocity: Velocity,
    pub collision_extent: CollisionExtent,
}

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct Velocity(pub Vec2);

impl Velocity {
    pub fn add_force(&mut self, force: Vec2) {
        self.0 += force;
    }
}

#[derive(Debug, Clone, Copy, Component)]
pub struct CollisionExtent(pub Vec2);

impl Default for CollisionExtent {
    fn default() -> Self {
        Self(Vec2::ONE)
    }
}

#[derive(Debug, Clone, Copy, Component)]
pub struct PhysicsBody {
    pub mass: f32,
    pub restitution: f32,
}

impl Default for PhysicsBody {
    fn default() -> Self {
        PhysicsBody {
            mass: 1.0,
            restitution: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, Component)]
pub struct Stable;

#[derive(Clone, Copy, Component, Debug, Default)]
pub struct RotatesTowardsDirection;
