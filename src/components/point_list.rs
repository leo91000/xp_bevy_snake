use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct PointList(pub Vec<Vec2>);

impl PointList {
    pub fn create_random_obstacle(max_distance: f32, point_count: u32, max_angle: f32) -> Self {
        let mut rng = rand::thread_rng();

        let mut points = Vec::new();
        let mut current_point = Vec2::ZERO;

        for _ in 0..point_count {
            let distance = rng.gen_range(0.0..max_distance);
            let angle = rng.gen_range(0.0..max_angle);

            current_point += Vec2::new(angle.cos(), angle.sin()) * distance;
            points.push(current_point);
        }

        Self(points)
    }
}
