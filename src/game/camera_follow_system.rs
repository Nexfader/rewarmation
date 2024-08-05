use bevy::prelude::*;

#[derive(Debug, Clone, Component, Copy)]
pub struct CameraFollowTarget;

pub fn do_camera_follow(
    mut cameras: Query<&mut Transform, With<Camera>>,
    follow_targets: Query<&GlobalTransform, With<CameraFollowTarget>>,
) {
    let Ok(mut camera_transform) = cameras.get_single_mut() else {
        return;
    };
    let Ok(transform) = follow_targets.get_single() else {
        return;
    };

    camera_transform.translation = transform.translation();
}
