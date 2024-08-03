use super::player::Player;
use crate::{physics::CollisionExtent, terrain::Terrain};
use bevy::prelude::*;

#[derive(Event)]
pub enum PlayerMovementEvent {
    Right,
    Left,
}

pub fn handle_player_movement(
    mut ev_movement: EventReader<PlayerMovementEvent>,
    time: Res<Time>,
    images: Res<Assets<Image>>,
    mut query: Query<
        (&mut Transform, &CollisionExtent, &mut Sprite),
        (With<Player>, Without<Terrain>),
    >,
    terrain_query: Query<(&Transform, &Handle<Image>), With<Terrain>>,
) {
    const WALK_SPEED: f32 = 60.0;
    const MAX_STEP_HEIGHT: u32 = 15;

    let Ok((terrain_transform, terrain_image)) = terrain_query.get_single() else {
        return;
    };
    let Some(terrain_image) = images.get(terrain_image) else {
        return;
    };

    for event in ev_movement.read() {
        for (mut transform, collision_extent, mut sprite) in &mut query {
            let (move_direction, flip) = match *event {
                PlayerMovementEvent::Left => (-1.0, true),
                PlayerMovementEvent::Right => (1.0, false),
            };
            sprite.flip_x = flip;

            let desired_position = Vec2::new(
                transform.translation.x + move_direction * WALK_SPEED * time.delta_seconds(),
                transform.translation.y,
            );
            for i in 0..MAX_STEP_HEIGHT {
                let desired_position = Vec2::new(desired_position.x, desired_position.y + i as f32);

                if Terrain::test_collision(
                    desired_position,
                    collision_extent.0,
                    &terrain_transform,
                    terrain_image,
                )
                .is_none()
                {
                    transform.translation = desired_position.extend(transform.translation.z);
                    break;
                }
            }
        }
    }
}
