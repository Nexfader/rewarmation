use crate::{
    player::{PlayerAimEvent, PlayerJumpEvent, PlayerMovementEvent},
    weapon::ChargeEvent,
};
use bevy::{app::AppExit, prelude::*};

pub fn handle_player_input(
    mut ev_movement: EventWriter<PlayerMovementEvent>,
    mut ev_jump: EventWriter<PlayerJumpEvent>,
    mut ev_aim: EventWriter<PlayerAimEvent>,
    mut ev_charge: EventWriter<ChargeEvent>,
    mut ev_exit: EventWriter<AppExit>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.pressed(KeyCode::ArrowRight) || keys.pressed(KeyCode::KeyD) {
        ev_movement.send(PlayerMovementEvent::Right);
    } else if keys.pressed(KeyCode::ArrowLeft) || keys.pressed(KeyCode::KeyA) {
        ev_movement.send(PlayerMovementEvent::Left);
    }

    if keys.just_pressed(KeyCode::Enter) {
        ev_jump.send(PlayerJumpEvent::Forward);
    }
    if keys.just_pressed(KeyCode::Backspace) {
        ev_jump.send(PlayerJumpEvent::Backward);
    }

    if keys.pressed(KeyCode::ArrowUp) || keys.pressed(KeyCode::KeyW) {
        ev_aim.send(PlayerAimEvent::Up);
    }
    if keys.pressed(KeyCode::ArrowDown) || keys.pressed(KeyCode::KeyS) {
        ev_aim.send(PlayerAimEvent::Down);
    }

    if keys.just_pressed(KeyCode::Space) {
        ev_charge.send(ChargeEvent::Start);
    }
    if keys.pressed(KeyCode::Space) {
        ev_charge.send(ChargeEvent::Charge);
    }
    if keys.just_released(KeyCode::Space) {
        ev_charge.send(ChargeEvent::End);
    }

    if keys.just_pressed(KeyCode::Escape) {
        ev_exit.send(AppExit::Success);
    }
}
