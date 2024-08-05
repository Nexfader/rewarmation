use super::{
    charge_system::ShootEvent, components::AimingAngle, projectile::ProjectileBundle, Weapon,
    WeaponBundle,
};
use crate::{
    game::{z_index, CameraFollowTarget, GamePlayState},
    physics::{CollisionExtent, PhysicsBodyBundle, Velocity},
};
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Bazooka;

#[derive(Bundle, Default)]
pub struct BazookaBundle {
    pub weapon: WeaponBundle,
    pub bazooka: Bazooka,
}

#[derive(Component)]
pub struct BazookaProjectile;

pub fn attach_bazooka(
    commands: &mut Commands,
    asset_server: &AssetServer,
    player: Entity,
) -> Entity {
    let bazooka = commands
        .spawn((
            Name::new("Bazooka"),
            StateScoped(GamePlayState::InGame),
            BazookaBundle {
                weapon: WeaponBundle {
                    weapon: Weapon { chargeable: true },
                    sprite: SpriteBundle {
                        transform: Transform::from_xyz(0.0, 0.0, z_index::WEAPON),
                        texture: asset_server.load("textures/bazooka.png"),
                        sprite: Sprite {
                            custom_size: Some(Vec2::splat(60.0)),
                            ..default()
                        },
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
        ))
        .id();
    commands.entity(player).add_child(bazooka);
    bazooka
}

pub fn handle_bazooka(
    mut commands: Commands,
    mut ev_shoot: EventReader<ShootEvent>,
    asset_server: Res<AssetServer>,
    weapons: Query<(&GlobalTransform, &AimingAngle), With<Bazooka>>,
) {
    const PROJECTILE_FORCE: f32 = 1000.0;
    const PROJECTILE_OFFSET: f32 = 10.0;

    for event in ev_shoot.read() {
        for (transform, aiming_angle) in &weapons {
            let direction = aiming_angle.direction();
            let position = transform.translation().xy() + direction * PROJECTILE_OFFSET;
            let force = direction * event.force * PROJECTILE_FORCE;

            spawn_bazooka_rocket(&mut commands, &asset_server, position, force);
        }
    }
}

fn spawn_bazooka_rocket(
    commands: &mut Commands,
    asset_server: &AssetServer,
    position: Vec2,
    force: Vec2,
) -> Entity {
    const PROJECTILE_EXTENT: Vec2 = Vec2::splat(60.0);
    const PROJECTILE_COLLISION_EXTENT: Vec2 = Vec2::splat(40.0);

    commands
        .spawn((
            Name::new("Bazooka projectile"),
            StateScoped(GamePlayState::InGame),
            BazookaProjectile,
            CameraFollowTarget,
            ProjectileBundle {
                sprite: SpriteBundle {
                    texture: asset_server.load("textures/bazookaRocket.png"),
                    sprite: Sprite {
                        custom_size: Some(PROJECTILE_EXTENT),
                        ..default()
                    },
                    transform: Transform::from_translation(position.extend(z_index::PROJECTILE))
                        .with_rotation(Quat::from_rotation_z(force.y.atan2(force.x))),
                    ..default()
                },
                body: PhysicsBodyBundle {
                    collision_extent: CollisionExtent(PROJECTILE_COLLISION_EXTENT),
                    velocity: Velocity(force),
                    ..default()
                },
                ..default()
            },
        ))
        .id()
}
