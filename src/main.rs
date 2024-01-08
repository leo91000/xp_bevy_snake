use bevy::{
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
    sprite::MaterialMesh2dBundle,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct Snake;

#[derive(Component)]
struct PointList(Vec<Vec2>);

#[derive(Component)]
struct Direction(Vec2);

fn setup(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let snake_point_list = [Vec2::new(0., 0.), Vec2::new(0., 1.), Vec2::new(0., 2.)];
    let _snake_direction = Vec2::new(0., 1.);

    let snake_vertices = snake_point_list
        .iter()
        .map(|point| Vec3::new(point.x, point.y, 0.))
        .collect::<Vec<_>>();

    let snake_indices: Vec<u32> = (0..snake_point_list.len() as u32).collect();

    let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);
    mesh.set_indices(Some(Indices::U32(snake_indices)));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, snake_vertices);
    let mesh = meshes.add(mesh);

    commands.spawn(MaterialMesh2dBundle {
        mesh: mesh.into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..Default::default()
    });

    commands.spawn(Camera2dBundle::default());
}
