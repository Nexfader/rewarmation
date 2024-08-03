use super::components::{ShootingForce, Weapon};
use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Event, PartialEq)]
pub enum ChargeEvent {
    Start,
    Charge,
    End,
}

#[derive(Default, Event)]
pub struct ShootEvent {
    pub force: f32,
}

pub fn handle_charging(
    time: Res<Time>,
    mut ev_charge: EventReader<ChargeEvent>,
    mut ev_shoot: EventWriter<ShootEvent>,
    mut weapons: Query<(&Weapon, &mut ShootingForce)>,
) {
    const SHOOTING_FORCE_STEP: f32 = 1.0;

    for event in ev_charge.read() {
        for (weapon, mut shooting_force) in &mut weapons {
            let ready_to_shoot = match *event {
                ChargeEvent::Start if !weapon.chargeable => true,
                ChargeEvent::Charge => {
                    shooting_force.0 += SHOOTING_FORCE_STEP * time.delta_seconds();
                    shooting_force.is_ready()
                }
                ChargeEvent::End => true,
                _ => false,
            };

            if ready_to_shoot {
                ev_shoot.send(ShootEvent {
                    force: shooting_force.0,
                });
                shooting_force.0 = 0.0;
            }
        }
    }
}
