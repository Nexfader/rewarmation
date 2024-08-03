mod player_input;

use crate::player::PlayerMovementSystemSet;
use bevy::prelude::*;
use player_input::handle_player_input;

#[derive(Debug, Clone, Copy, SystemSet, PartialEq, Eq, Hash)]
pub struct InputSystemSet;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_player_input)
                .in_set(InputSystemSet)
                .before(PlayerMovementSystemSet),
        );
    }
}
