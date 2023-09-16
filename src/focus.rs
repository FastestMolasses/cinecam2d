use bevy::prelude::*;

use super::MainCameraTag;

#[derive(Resource, Default, Clone)]
pub struct CameraFocusConfig {
    /// The offset from the target to focus on
    pub offset: Vec3,
    /// The speed at which the camera will move to the target. 0.0 is instant, 1.0 is never
    pub lerp: f32,
}

/// The camera will attempt to focus on all entities with this component
#[derive(Component)]
pub struct FocusTarget;

pub fn focus_target(
    camera_focus_config: Option<Res<CameraFocusConfig>>,
    focus_target_query: Query<&Transform, (With<FocusTarget>, Without<MainCameraTag>)>,
    mut camera_query: Query<(&MainCameraTag, &mut Transform)>,
) {
    // If there's no config, use a default one
    let camera_focus_config = camera_focus_config.map_or_else(
        || CameraFocusConfig::default(), // Default if None
        |res| res.clone()
    );

    // Get the camera's Transform
    let mut camera_transform = match camera_query.iter_mut().next() {
        Some((_tag, transform)) => transform,
        None => return,
    };

    // Calculate the average position of all focus targets
    let mut target_position_sum = Vec3::ZERO;
    let mut target_count = 0;
    for transform in focus_target_query.iter() {
        target_position_sum += transform.translation;
        target_count += 1;
    }

    if target_count == 0 {
        return;
    }

    let average_target_position = target_position_sum / target_count as f32;
    // Apply the offset to the average target position
    let target_position = average_target_position + camera_focus_config.offset;

    let new_camera_position = camera_transform
        .translation
        .lerp(target_position, 1.0 - camera_focus_config.lerp);
    camera_transform.translation = new_camera_position;

    // TODO: ADJUST CAMERA ZOOM TO FIT ALL TARGETS
}
