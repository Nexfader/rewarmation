mod aiming_system;
mod jumping_system;
mod movement_system;
mod player;

pub use aiming_system::PlayerAimEvent;
pub use jumping_system::PlayerJumpEvent;
pub use movement_system::PlayerMovementEvent;
pub use player::{spawn_player, Player, PlayerBundle};

use crate::{
    game::{GamePlayState, PlayingState},
    physics::PhysicsSystemSet,
};
use aiming_system::handle_player_aiming;
use bevy::prelude::*;
use jumping_system::handle_player_jumps;
use movement_system::handle_player_movement;

pub struct PlayerPlugin;

#[derive(Debug, Clone, Copy, SystemSet, PartialEq, Eq, Hash)]
pub struct PlayerMovementSystemSet;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (handle_player_movement, handle_player_jumps)
                .in_set(PlayerMovementSystemSet)
                .before(PhysicsSystemSet)
                .run_if(in_state(GamePlayState::InGame))
                .run_if(in_state(PlayingState::Playing)),
        )
        .add_systems(
            Update,
            (handle_player_aiming)
                .run_if(in_state(GamePlayState::InGame))
                .run_if(in_state(PlayingState::Playing)),
        )
        .add_event::<PlayerMovementEvent>()
        .add_event::<PlayerJumpEvent>()
        .add_event::<PlayerAimEvent>();
    }
}
