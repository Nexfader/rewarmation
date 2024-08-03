use super::components::{ShootingForce, Weapon};
use crate::game::z_index;
use bevy::{prelude::*, sprite::Anchor};

#[derive(Clone, Copy, Component)]
pub struct Charger;

fn spawn_charger(commands: &mut Commands, asset_server: &AssetServer, weapon: Entity) -> Entity {
    const CHARGER_SIZE: f32 = 100.0;
    const CHARGER_OFFSET: f32 = 50.0;

    let aimer = commands
        .spawn((
            Charger,
            SpriteBundle {
                visibility: Visibility::Hidden,
                texture: asset_server.load("textures/charger.png"),
                transform: Transform::from_xyz(CHARGER_OFFSET, 0.0, z_index::CHARGER),
                sprite: Sprite {
                    anchor: Anchor::CenterLeft,
                    custom_size: Some(Vec2::splat(CHARGER_SIZE)),
                    ..default()
                },
                ..default()
            },
        ))
        .id();
    commands.entity(weapon).add_child(aimer);
    aimer
}

pub fn render_chargers(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    weapons: Query<(Entity, &ShootingForce, Option<&Children>), With<Weapon>>,
    mut chargers: Query<(&mut Transform, &mut Visibility), (With<Charger>, Without<Weapon>)>,
) {
    for (weapon_entity, shooting_force, weapon_children) in &weapons {
        let charger = weapon_children
            .and_then(|children| {
                children
                    .iter()
                    .copied()
                    .find(|child| chargers.contains(*child))
            })
            .unwrap_or_else(|| spawn_charger(&mut commands, &asset_server, weapon_entity));

        let Ok((mut charger_transform, mut charger_visibility)) = chargers.get_mut(charger) else {
            continue;
        };

        if shooting_force.is_empty() {
            *charger_visibility = Visibility::Hidden;
            continue;
        }

        *charger_visibility = Visibility::Visible;
        charger_transform.scale = Vec3::splat(shooting_force.0);
    }
}
