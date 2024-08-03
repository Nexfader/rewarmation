use crate::{
    game::{z_index, Health},
    physics::{CollisionExtent, PhysicsBody, PhysicsBodyBundle},
};
use bevy::prelude::*;

#[derive(Clone, Component, Copy, Debug, Default)]
pub struct Player;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub player: Player,
    pub health: Health,
    pub sprite: SpriteBundle,
    pub physics_body: PhysicsBodyBundle,
}

pub fn spawn_player(commands: &mut Commands, asset_server: &AssetServer, position: Vec2) -> Entity {
    commands
        .spawn((
            Name::new("Player"),
            PlayerBundle {
                sprite: SpriteBundle {
                    texture: asset_server.load("textures/player.png"),
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat(100.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(position.extend(z_index::PLAYER)),
                    ..default()
                },
                physics_body: PhysicsBodyBundle {
                    collision_extent: CollisionExtent(Vec2::splat(60.0)),
                    body: PhysicsBody {
                        mass: 1.0,
                        restitution: 0.2,
                    },
                    ..default()
                },
                ..default()
            },
        ))
        .id()
}
