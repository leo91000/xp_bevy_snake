use bevy::color::palettes::tailwind;
use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::sprite::MaterialMesh2dBundle;

use super::point_list::PointList;

#[derive(Component)]
pub struct Obstacle;

impl Obstacle {
    /// Create the polygon mesh triangle from point list
    fn get_indices_and_vertices(point_list: &PointList) -> (Vec<u32>, Vec<[f32; 3]>) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        if point_list.0.len() < 3 {
            return (indices, vertices);
        }

        for i in 0..point_list.0.len() {
            vertices.push([point_list.0[i].x, point_list.0[i].y, 1.0]);
        }

        for i in 1..(point_list.0.len() - 1) {
            indices.extend_from_slice(&[0, i as u32, (i + 1) as u32]);
        }

        (indices, vertices)
    }

    fn create_mesh(point_list: &PointList) -> Mesh {
        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleStrip,
            RenderAssetUsages::default(),
        );
        let (indices, vertices) = Self::get_indices_and_vertices(point_list);

        mesh.insert_indices(Indices::U32(indices));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);

        mesh
    }

    pub fn create_from_point_list(
        points: PointList,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        meshes: &mut ResMut<Assets<Mesh>>,
    ) -> (Self, PointList, MaterialMesh2dBundle<ColorMaterial>) {
        let mesh = meshes.add(Self::create_mesh(&points)).into();
        let material = materials.add(Color::from(tailwind::RED_500));

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
