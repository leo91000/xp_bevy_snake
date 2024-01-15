use bevy::prelude::*;

/// Represents the angle of the snake in radians
#[derive(Component)]
pub struct Direction(pub f32);

impl Default for Direction {
    fn default() -> Self {
        Self(std::f32::consts::PI)
    }
}
