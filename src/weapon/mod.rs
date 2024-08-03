mod aiming_angle_system;
mod bazooka;
mod charge_system;
mod charger_render_system;
mod components;
mod projectile;
mod projectile_explode_system;

pub use bazooka::{attach_bazooka, Bazooka, BazookaBundle};
pub use charge_system::ChargeEvent;
pub use components::{AimingAngle, Weapon, WeaponBundle};

use crate::{
    game::{GamePlayState, PlayingState},
    physics::PhysicsSystemSet,
};
use aiming_angle_system::apply_aiming_angle;
use bazooka::handle_bazooka;
use bevy::prelude::*;
use charge_system::{handle_charging, ShootEvent};
use charger_render_system::render_chargers;
use projectile_explode_system::explode_projectiles;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                apply_aiming_angle,
                handle_charging,
                handle_bazooka,
                explode_projectiles,
                render_chargers,
            )
                .before(PhysicsSystemSet)
                .run_if(in_state(GamePlayState::InGame))
                .run_if(in_state(PlayingState::Playing)),
        )
        .add_event::<ChargeEvent>()
        .add_event::<ShootEvent>();
    }
}
