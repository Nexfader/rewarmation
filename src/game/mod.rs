mod components;
mod explosion_system;
mod state;

pub mod z_index;

pub use components::Health;
pub use explosion_system::ExplosionEvent;
pub use state::{GameEndKind, GamePlayState, GameState, PlayingState};

use bevy::prelude::*;
use explosion_system::handle_explosions;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_explosions)
                .run_if(in_state(GamePlayState::InGame))
                .run_if(in_state(PlayingState::Playing)),
        )
        .add_event::<ExplosionEvent>();
    }
}
