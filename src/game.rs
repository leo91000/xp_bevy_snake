use bevy::{prelude::*, render::mesh::Indices, sprite::Mesh2dHandle};

use crate::app_state::AppState;
use crate::components::{
    direction::Direction, obstacle::Obstacle, point_list::PointList, snake::Snake,
};
use crate::{
    consts::{DISTANCE_BETWEEN_POINTS, MOVEMENT_SPEED, TURN_SPEED},
    load_level,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        let snake_system_set = (update_direction, update_position, update_mesh);
        app.add_systems(OnEnter(AppState::InGame), setup)
            .add_systems(OnExit(AppState::InGame), (cleanup_snake, cleanup_obstacles))
            .add_systems(Update, snake_system_set.run_if(in_state(AppState::InGame)));
    }
}

fn setup(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Snake::get_default_entity_components(
        &mut meshes,
        &mut materials,
    ));

    let simple_level: crate::levels::Level = load_level!("simple");

    for obstacle in simple_level.obstacles {
        commands.spawn(Obstacle::create_from_point_list(
            obstacle,
            &mut materials,
            &mut meshes,
        ));
    }
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

    let (indices, vertices) = Snake::get_indices_and_vertices(point_list);

    mesh.insert_indices(Indices::U32(indices));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
}

fn update_direction(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Direction, With<Snake>>,
) {
    let mut direction = query.single_mut();

    let mut offset = None;
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        offset = Some(TURN_SPEED);
    } else if keyboard_input.pressed(KeyCode::ArrowRight) {
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
}

fn cleanup_snake(mut commands: Commands, query: Query<Entity, With<Snake>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn cleanup_obstacles(mut commands: Commands, query: Query<Entity, With<Obstacle>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
