use bevy::{
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (update_direction, update_position, update_mesh))
        .run();
}

#[derive(Component)]
struct Snake;

#[derive(Component)]
struct PointList(Vec<Vec2>);

impl PointList {
    fn get_indices_and_vertices(&self) -> (Vec<u32>, Vec<[f32; 3]>) {
        let mut indices = Vec::new();
        let mut vertices = Vec::new();

        for (i, point) in self.0.iter().enumerate() {
            vertices.push([point.x, point.y, 0.0]);
            indices.push(i as u32);
        }

        (indices, vertices)
    }

    fn mesh(&self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);
        let (indices, vertices) = self.get_indices_and_vertices();

        mesh.set_indices(Some(Indices::U32(indices)));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);

        mesh
    }
}

#[derive(Component)]
struct Direction(Vec2);

fn setup(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let snake = Snake;
    let point_list = PointList(vec![
        Vec2::new(0.0, 0.0),
        Vec2::new(10.0, 0.0),
        Vec2::new(20.0, 0.0),
    ]);
    let direction = Direction(Vec2::new(0.0, 10.0));

    let mesh = meshes.add(point_list.mesh());

    commands.spawn((
        snake,
        point_list,
        direction,
        MaterialMesh2dBundle {
            mesh: mesh.into(),
            material: materials.add(Color::RED.into()),
            ..Default::default()
        },
    ));

    commands.spawn(Camera2dBundle::default());
}

fn update_mesh(
    mut query: Query<(&Snake, &PointList, &mut Mesh2dHandle)>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let Ok((_, point_list, mesh_handle)) = query.get_single_mut() else {
        return;
    };

    let Some(mesh) = meshes.get_mut(mesh_handle.0.id()) else {
        return;
    };

    let (indices, vertices) = point_list.get_indices_and_vertices();

    mesh.set_indices(Some(Indices::U32(indices)));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
}

fn update_direction(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Direction>) {
    let mut direction = query.single_mut();

    // Move the direction within a circle of radius 10
    if keyboard_input.just_pressed(KeyCode::Left) {
        direction.0 = Vec2::new(-10.0, 0.0);
    } else if keyboard_input.just_pressed(KeyCode::Right) {
        direction.0 = Vec2::new(10.0, 0.0);
    } else if keyboard_input.just_pressed(KeyCode::Up) {
        direction.0 = Vec2::new(0.0, 10.0);
    } else if keyboard_input.just_pressed(KeyCode::Down) {
        direction.0 = Vec2::new(0.0, -10.0);
    }
}

/// We want to have a snake effect
fn update_position(time: Res<Time>, mut query: Query<(&mut PointList, &Direction)>) {
    let dt = time.delta_seconds();

    for (mut point_list, direction) in query.iter_mut() {
        let mut new_point_list = Vec::new();

        point_list.0 = new_point_list;
        dbg!(&point_list.0);
    }
}
