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

const DISTANCE_BETWEEN_POINTS: f32 = 3.0;
const INITIAL_LENGTH: f32 = 20.0;
const SNAKE_THICKNESS: f32 = 3.0;

impl PointList {
    fn get_indices_and_vertices(&self) -> (Vec<u32>, Vec<[f32; 3]>) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for i in 0..self.0.len() {
            let point = self.0[i];
            let normal = if i < self.0.len() - 1 {
                // Calculate normal for the current segment
                (self.0[i + 1] - point).perp().normalize() * SNAKE_THICKNESS / 2.0
            } else {
                // Use the normal from the previous segment for the last point
                (point - self.0[i - 1]).perp().normalize() * SNAKE_THICKNESS / 2.0
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

    fn mesh(&self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleStrip);
        let (indices, vertices) = self.get_indices_and_vertices();

        mesh.set_indices(Some(Indices::U32(indices)));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);

        mesh
    }

    /// Creates a default point list (a line on the x axis)
    fn create_default() -> Self {
        let mut points = Vec::new();

        for i in 0..(INITIAL_LENGTH as u32) {
            points.push(Vec2::new(i as f32 * DISTANCE_BETWEEN_POINTS, 0.0));
        }

        Self(points)
    }
}

/// Represents the angle of the snake in radians
#[derive(Component)]
struct Direction(f32);

fn setup(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let snake = Snake;
    let point_list = PointList::create_default();
    let direction = Direction(std::f32::consts::PI);

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

const TURN_SPEED: f32 = 3.0;

fn update_direction(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Direction, With<Snake>>,
) {
    let mut direction = query.single_mut();

    let mut offset = None;
    if keyboard_input.pressed(KeyCode::Left) {
        offset = Some(TURN_SPEED);
    } else if keyboard_input.pressed(KeyCode::Right) {
        offset = Some(-TURN_SPEED);
    }

    if let Some(offset) = offset {
        direction.0 += offset * time.delta_seconds();
        if direction.0 < 0.0 {
            direction.0 += std::f32::consts::TAU;
        } else if direction.0 > std::f32::consts::TAU {
            direction.0 -= std::f32::consts::TAU;
        }
    }
}

const MOVEMENT_SPEED: f32 = 20.0;

/// We want to have a snake effect
fn update_position(time: Res<Time>, mut query: Query<(&mut PointList, &Direction), With<Snake>>) {
    let dt = time.delta_seconds();

    let Ok((mut point_list, direction)) = query.get_single_mut() else {
        return;
    };

    // Move the head of the snake
    let head_movement = Vec2::new(direction.0.cos(), direction.0.sin()) * MOVEMENT_SPEED * dt;
    point_list.0[0] += head_movement;

    // Update the positions of the other points
    for i in 1..point_list.0.len() {
        let prev_point = point_list.0[i - 1];
        let current_point = &mut point_list.0[i];

        // Calculate the distance and direction to the previous point
        let distance_to_prev = current_point.distance(prev_point);
        let dir_to_prev = (prev_point - *current_point).normalize_or_zero();

        // Move the current point towards the previous point if it's too far
        if distance_to_prev > DISTANCE_BETWEEN_POINTS {
            *current_point += dir_to_prev * MOVEMENT_SPEED * dt;
        }
    }

    // 5% chance to panic
    // dbg!(&point_list.0);
    // if rand::random::<f32>() < 0.05 {
    //     panic!("");
    // }
}
