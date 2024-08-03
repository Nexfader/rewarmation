use super::Player;
use crate::physics::{Stable, Velocity};
use bevy::prelude::*;

#[derive(Event)]
pub enum PlayerJumpEvent {
    Forward,
    Backward,
}

pub fn handle_player_jumps(
    mut ev_jump: EventReader<PlayerJumpEvent>,
    mut query: Query<(&mut Velocity, &mut Sprite), (With<Player>, With<Stable>)>,
) {
    const FORWARD_JUMP_FORCE: Vec2 = Vec2::splat(400.0);
    const BACKWARD_JUMP_FORCE: Vec2 = Vec2::new(150.0, 600.0);

    for event in ev_jump.read() {
        for (mut velocity, sprite) in &mut query {
            let direction = if !sprite.flip_x { 1.0 } else { -1.0 };
            let force = match *event {
                PlayerJumpEvent::Forward => FORWARD_JUMP_FORCE * Vec2::new(direction, 1.0),
                PlayerJumpEvent::Backward => BACKWARD_JUMP_FORCE * Vec2::new(-direction, 1.0),
            };
            velocity.add_force(force);
        }
    }
}
