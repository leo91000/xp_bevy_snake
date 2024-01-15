use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::sprite::MaterialMesh2dBundle;

use crate::consts::{DISTANCE_BETWEEN_POINTS, INITIAL_LENGTH, SNAKE_THICKNESS};

use super::direction::Direction;
use super::point_list::PointList;

#[derive(Component)]
pub struct Snake;

impl Snake {
    pub fn get_indices_and_vertices(point_list: &PointList) -> (Vec<u32>, Vec<[f32; 3]>) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for i in 0..point_list.0.len() {
            let point = point_list.0[i];
            let normal = if i < point_list.0.len() - 1 {
                // Calculate normal for the current segment
                (point_list.0[i + 1] - point).perp().normalize() * SNAKE_THICKNESS / 2.0
            } else {
                // Use the normal from the previous segment for the last point
                (point - point_list.0[i - 1]).perp().normalize() * SNAKE_THICKNESS / 2.0
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

    pub fn create_mesh(point_list: &PointList) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleStrip);
        let (indices, vertices) = Self::get_indices_and_vertices(point_list);

        mesh.set_indices(Some(Indices::U32(indices)));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);

        mesh
    }

    pub fn get_default_entity_components(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> (
        Self,
        PointList,
        Direction,
        MaterialMesh2dBundle<ColorMaterial>,
    ) {
        let mut points = Vec::new();

        for i in 0..INITIAL_LENGTH {
            points.push(Vec2::new(i as f32 * DISTANCE_BETWEEN_POINTS, 0.0));
        }

        let point_list = PointList(points);
        let mesh = Self::create_mesh(&point_list);

        (
            Snake,
            point_list,
            Direction::default(),
            MaterialMesh2dBundle {
                mesh: meshes.add(mesh).into(),
                material: materials.add(Color::RED.into()),
                ..Default::default()
            },
        )
    }
}
