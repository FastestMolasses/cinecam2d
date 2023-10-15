use bevy::prelude::*;
use cinecam2d::CineCam2DPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CineCam2DPlugin)
        .add_systems(Startup, world_setup)
        .run();
}

fn world_setup(mut commands: Commands) {
    cinecam2d::init(&mut commands, Transform::from_xyz(0.0, 0.0, 10.0));
}
