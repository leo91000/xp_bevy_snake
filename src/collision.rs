use bevy::prelude::*;

use crate::{
    app_state::AppState,
    components::{obstacle::Obstacle, point_list::PointList, snake::Snake},
    players_lifes::{setup_players_lifes, PlayersLifes},
};

#[derive(Component)]
pub struct InvincibilityTimer {
    pub timer: Timer,
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (collision_system, invincibility_timer_system)
                .after(setup_players_lifes)
                .run_if(in_state(AppState::InGame)),
        );
    }
}

type SnakeQuery<'world, 'state, 'point_list> =
    Query<'world, 'state, (&'point_list mut PointList, Entity), (With<Snake>, Without<Obstacle>)>;

type ObstacleQuery<'world, 'state, 'point_list> =
    Query<'world, 'state, (&'point_list PointList, Entity), (With<Obstacle>, Without<Snake>)>;

pub fn collision_system(
    mut commands: Commands,
    mut snake_query: SnakeQuery,
    mut obstacle_query: ObstacleQuery,
    mut lifes_query: ResMut<PlayersLifes>,
    invincibility_query: Query<&InvincibilityTimer>,
) {
    if !invincibility_query.is_empty() {
        return;
    }

    let (mut snake_point_list, _) = snake_query.single_mut();

    for (obstacle_point_list, _) in obstacle_query.iter_mut() {
        if point_in_polygon(&snake_point_list.0[0], &obstacle_point_list.0) {
            // Remove one life
            // Add invincibility for 3 seconds
            // Remove N points from the snake
            lifes_query.0 -= 1;
            commands.spawn(InvincibilityTimer {
                timer: Timer::from_seconds(3.0, TimerMode::Once),
            });
            let snake_length = snake_point_list.0.len();
            snake_point_list.truncate(snake_length - 3);

            break;
        }
    }
}

fn invincibility_timer_system(
    mut commands: Commands,
    time: Res<Time>,
    mut invincibility_query: Query<(Entity, &mut InvincibilityTimer)>,
) {
    for (entity, mut invincibility_timer) in invincibility_query.iter_mut() {
        invincibility_timer.timer.tick(time.delta());
        if invincibility_timer.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn point_in_polygon(point: &Vec2, polygon: &[Vec2]) -> bool {
    let mut is_inside = false;
    let mut j = polygon.len() - 1;

    for i in 0..polygon.len() {
        let pi = polygon[i];
        let pj = polygon[j];

        if ((pi.y > point.y) != (pj.y > point.y))
            && (point.x < (pj.x - pi.x) * (point.y - pi.y) / (pj.y - pi.y) + pi.x)
        {
            is_inside = !is_inside;
        }

        j = i;
    }

    is_inside
}
