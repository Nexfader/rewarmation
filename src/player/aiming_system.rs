use super::Player;
use crate::{
    physics::Stable,
    weapon::{AimingAngle, Weapon},
};
use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Event)]
pub enum PlayerAimEvent {
    Up,
    Down,
}

pub fn handle_player_aiming(
    time: Res<Time>,
    mut ev_aim: EventReader<PlayerAimEvent>,
    players: Query<(&Sprite, &Children), (With<Player>, With<Stable>)>,
    mut weapons: Query<(&mut Sprite, &mut AimingAngle), (Without<Player>, With<Weapon>)>,
) {
    const AIMING_SPEED: f32 = 3.0;

    // Currently, we have to manually propagate flipping through hierarchy
    // Tracking issue: https://github.com/bevyengine/bevy/issues/4930
    for (player_sprite, children) in &players {
        let mut iter = weapons.iter_many_mut(children);
        while let Some((mut weapon_sprite, mut aiming_angle)) = iter.fetch_next() {
            let flip = player_sprite.flip_x;
            weapon_sprite.flip_y = flip;
            if !aiming_angle.is_in_bounds(flip) {
                aiming_angle.flip();
            }
        }
    }

    for event in ev_aim.read() {
        for (sprite, mut aiming_angle) in &mut weapons {
            let mut direction = match *event {
                PlayerAimEvent::Up => 1.0,
                PlayerAimEvent::Down => -1.0,
            };
            if sprite.flip_y {
                direction *= -1.0;
            }

            aiming_angle.0 += direction * AIMING_SPEED * time.delta_seconds();
            aiming_angle.clamp_to_bounds(sprite.flip_y);
        }
    }
}
