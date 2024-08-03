use crate::physics::{PhysicsBodyBundle, RotatesTowardsDirection};
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Projectile;

#[derive(Clone, Copy, Component, Debug, Default)]
pub struct ExplodesOnTerrainCollision;

#[derive(Bundle, Default)]
pub struct ProjectileBundle {
    pub projectile: Projectile,
    pub explode: ExplodesOnTerrainCollision,
    pub rotate: RotatesTowardsDirection,
    pub sprite: SpriteBundle,
    pub body: PhysicsBodyBundle,
}
