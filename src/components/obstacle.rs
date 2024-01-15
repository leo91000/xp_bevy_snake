use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::sprite::MaterialMesh2dBundle;
use rand::Rng;

use super::point_list::PointList;

#[derive(Component)]
pub struct Obstacle;

impl Obstacle {
    /// Create the polygon mesh triangle from point list
    fn get_indices_and_vertices(point_list: &PointList) -> (Vec<u32>, Vec<[f32; 3]>) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for i in 0..point_list.0.len() {
            let point = point_list.0[i];
            let normal = if i < point_list.0.len() - 1 {
                // Calculate normal for the current segment
                (point_list.0[i + 1] - point).perp().normalize() * 0.5
            } else {
                // Use the normal from the previous segment for the last point
                (point - point_list.0[i - 1]).perp().normalize() * 0.5
            };

            // Add two vertices for the edges of the snake
            vertices.push([point.x - normal.x, point.y - normal.y, 0.0]);
            vertices.push([point.x + normal.x, point.y + normal.y, 0.0]);
        }

        // Create indices for the triangle strip
        for i in 0..(vertices.len() as u32 / 2 - 1) {
            let base = i * 2;
            indices.extend_from_slice(&[base, base + 1, base + 2, base + 2, base + 1, base + 3]);
        }

        (indices, vertices)
    }

    fn create_mesh(point_list: &PointList) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleStrip);
        let (indices, vertices) = Self::get_indices_and_vertices(point_list);

        mesh.set_indices(Some(Indices::U32(indices)));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);

        mesh
    }

    pub fn create_random(
        max_distance: f32,
        max_angle: f32,
        min_points: u32,
        max_points: u32,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        meshes: &mut ResMut<Assets<Mesh>>,
    ) -> (Self, PointList, MaterialMesh2dBundle<ColorMaterial>) {
        let mut rng = rand::thread_rng();
        let mut points = PointList::new();
        let mut vertices = Vec::new();

        let mut last_point = Vec2::new(0.0, 0.0);
        let mut last_direction = Vec2::new(0.0, 1.0);

        let points_count = rng.gen_range(min_points..max_points);
        for _ in min_points..max_points - points_count {
            let distance = rng.gen_range(0.0..max_distance);
            let angle = rng.gen_range(-max_angle..max_angle);

            let angle_vec = Vec2::new(angle.cos(), angle.sin());
            let direction = last_direction.rotate(angle_vec);
            let point = last_point + direction * distance;

            points.push(point);
            vertices.push(point);
            last_point = point;
            last_direction = direction;
        }

        let mesh = meshes.add(Self::create_mesh(&points)).into();
        let material = materials.add(Color::RED.into());

        (
            Self,
            points,
            MaterialMesh2dBundle {
                material,
                mesh,
                ..Default::default()
            },
        )
    }
}
