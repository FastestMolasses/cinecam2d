/// Credit: https://github.com/Andrewp2/bevy_camera_shake/blob/main/src/lib.rs

use bevy::{
    prelude::{warn, Component, EulerRot, Quat, Query, Res, Transform, Vec2, Vec3},
    time::Time,
};
use noise::{NoiseFn, OpenSimplex};

/// A source of randomness for shaking the camera.
pub trait RandomSource: Send + Sync {
    /// Produces a random float between -1.0 and 1.0.
    fn rand(&self, time: f32) -> f32;
}

/// A not-random RandomSource. Always returns 0.5.
struct NotRandom;

impl RandomSource for NotRandom {
    fn rand(&self, _time: f32) -> f32 {
        warn!("You need to set a random source for the shaking to work properly!");
        0.5
    }
}

/// Will modify the transform of the entity that this component is added to.
/// The entity must also have a `Transform` component for the shake to work.
#[derive(Component)]
pub struct CameraShake {
    /// The maximum amount of offset in the X and Y dimensions.
    /// Defaults to `Vec2::new(100.0, 100.0)`.
    pub max_offset: Vec2,
    /// The maximum amount of roll allowed in radians.
    /// Defaults to `0.1`.
    pub max_roll: f32,
    /// The starting trauma when created.
    /// Defaults to `0.0`.
    pub trauma: f32,
    /// The exponent of the trauma used when calculating offset and rotational shakiness.
    /// Should likely be set to a value between `2.0` and `3.0`.
    /// Defaults to `2.0`.
    pub trauma_power: f32,
    /// The percentage to decrease trauma per second.
    /// If set to 1, there will be no trauma after 1 second. If set to 0, trauma will not decrease over time.
    /// If set below 0, trauma will *increase* over time, and if set above 1, trauma will decrease very quickly.
    /// Defaults to `0.8`.
    pub decay: f32,
    /// The random sources for all 3 dimensions.
    /// The first 2 are for XY lateral motion, the last one is for roll.
    /// Defaults to a `NotRandom`, which always returns `0.5`.
    pub random_sources: [Box<dyn RandomSource>; 3],
}

impl Default for CameraShake {
    fn default() -> Self {
        Self {
            max_offset: Vec2::new(100.0, 100.0),
            max_roll: 0.1,
            trauma: 0.0,
            trauma_power: 2.0,
            decay: 0.8,
            random_sources: [
                Box::new(NotRandom),
                Box::new(NotRandom),
                Box::new(NotRandom),
            ],
        }
    }
}

pub fn apply_shake_2d(mut query: Query<(&mut Transform, &mut CameraShake)>, time: Res<Time>) {
    for (mut transform, mut shake_settings) in query.iter_mut() {
        shake_settings.trauma = f32::max(
            shake_settings.trauma - shake_settings.decay * time.delta_seconds(),
            0.0,
        );

        let trauma_amount = f32::powf(shake_settings.trauma, shake_settings.trauma_power);
        if trauma_amount > 0.0 {
            let offset = shake_settings.max_offset
                * trauma_amount
                * Vec2::new(
                    shake_settings.random_sources[0].rand(time.elapsed_seconds()),
                    shake_settings.random_sources[1].rand(time.elapsed_seconds()),
                );

            let shake_translation = Vec3::new(offset.x, offset.y, 0.0);

            let shake_rotation = Quat::from_euler(
                EulerRot::YXZ,
                0.0,
                0.0,
                shake_settings.max_roll
                    * trauma_amount
                    * shake_settings.random_sources[2].rand(time.elapsed_seconds()),
            );
            transform.translation = shake_translation;
            transform.rotation = shake_rotation;
        } else {
            transform.translation = Vec3::default();
            transform.rotation = Quat::default();
        }
    }
}

// Struct used to hold our simplex noise.
// We have to use a seperate struct (rather than implementing `RandomSource` on `OpenSimplex` directly)
// due to https://doc.rust-lang.org/error_codes/E0117.html
struct MyNoise {
    pub simplex: OpenSimplex,
}

impl MyNoise {
    pub fn new(seed: u32) -> MyNoise {
        MyNoise {
            simplex: OpenSimplex::new(seed),
        }
    }
}

impl RandomSource for MyNoise {
    fn rand(&self, time: f32) -> f32 {
        self.simplex.get([(time * 15.0) as f64, 0.0]) as f32
    }
}
