use bevy::prelude::*;

use super::MainCameraTag;

#[derive(Resource)]
pub struct CameraBoundBoxConfig {
    /// The minimum position of the camera
    pub min: Vec3,
    /// The maximum position of the camera
    pub max: Vec3,
}

pub fn bounding_box(
    bound_box_config: Option<Res<CameraBoundBoxConfig>>,
    mut camera_query: Query<(&MainCameraTag, &mut Transform)>,
) {
    // If the bounding box config is not provided, early return
    let bound_box_config = match bound_box_config {
        Some(config) => config,
        None => return,
    };

    let mut camera_transform = match camera_query.iter_mut().next() {
        Some((_tag, transform)) => transform,
        None => return,
    };

    // Create a mutable reference for easier access
    let camera_translation = &mut camera_transform.translation;
    camera_translation.x = camera_translation
        .x
        .max(bound_box_config.min.x)
        .min(bound_box_config.max.x);
    camera_translation.y = camera_translation
        .y
        .max(bound_box_config.min.y)
        .min(bound_box_config.max.y);
    camera_translation.z = camera_translation
        .z
        .max(bound_box_config.min.z)
        .min(bound_box_config.max.z);
}
