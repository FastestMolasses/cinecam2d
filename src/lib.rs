pub mod focus;
#[cfg(feature = "bound")]
pub mod bound_box;
#[cfg(feature = "pan")]
pub mod pan;
#[cfg(feature = "shake")]
pub mod shake;
#[cfg(feature = "zoom")]
pub mod zoom;

use bevy::prelude::*;

// TODO: PIXEL PERFECT MODE
// TODO: RETRO PIXEL DOWNSCALE MODE

pub struct CineCam2DPlugin;

impl Plugin for CineCam2DPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, focus::focus_target);

        #[cfg(feature = "bound")]
        app.add_systems(Update, bound_box::bounding_box);

        #[cfg(feature = "pan")]
        app.add_systems(Update, pan::pan);

        #[cfg(feature = "shake")]
        app.add_systems(Update, zoom::zoom);

        #[cfg(feature = "shake")]
        app.add_systems(Update, shake::apply_shake_2d);
    }
}

#[derive(Component)]
pub struct MainCameraTag;

pub fn init(mut commands: Commands, transform: Transform) {
    commands.spawn((
        Camera2dBundle {
            transform,
            ..Default::default()
        },
        MainCameraTag,
    ));
}
