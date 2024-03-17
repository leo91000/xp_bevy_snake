use bevy::prelude::*;
use rand::Rng;

use crate::{
    app_state::AppState,
    components::{point_list::PointList, snake::Snake},
};

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_food)
            .add_systems(OnExit(AppState::InGame), cleanup_foods)
            .add_systems(
                Update,
                food_collision_system.run_if(in_state(AppState::InGame)),
            );
    }
}

#[derive(Component)]
pub struct Food;

const INITIAL_FOOD_COUNT: u32 = 4;

pub fn spawn_food(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_food_count(&mut commands, &asset_server, INITIAL_FOOD_COUNT);
}

fn spawn_food_count(commands: &mut Commands, asset_server: &Res<AssetServer>, count: u32) {
    for i in 0..count {
        let x = (i % 2) as f32 * 100.0 - 50.0;
        let y = (i / 2) as f32 * 100.0 - 50.0;

        // Randomize food position
        let mut rng = rand::thread_rng();
        let x = x + rng.gen_range(-30.0..30.0);
        let y = y + rng.gen_range(-30.0..30.0);

        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("images/foods/icon_0_0.png"),
                transform: Transform {
                    translation: Vec3::new(x, y, 0.0),
                    scale: Vec3::new(0.5, 0.5, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            Food,
        ));
    }
}

fn food_collision_system(
    mut commands: Commands,
    mut food_query: Query<(Entity, &Transform), With<Food>>,
    mut snake_query: Query<&mut PointList, With<Snake>>,
    asset_server: Res<AssetServer>,
) {
    for (food_entity, food_transform) in food_query.iter_mut() {
        for mut point_list in snake_query.iter_mut() {
            let distance = point_list.0[0].distance(food_transform.translation.xy());

            if distance < 10.0 {
                commands.entity(food_entity).despawn();

                // Add 3 points to the snake in the direction of the last 2 point
                let last_point = point_list
                    .0
                    .last()
                    .expect("Snake should have at least 1 point");
                let second_last_point = point_list
                    .0
                    .get(point_list.0.len() - 2)
                    .expect("Snake should have at least 2 points");
                let direction = *last_point - *second_last_point;
                let p1 = *last_point + direction;
                let p2 = *last_point + direction * 2.0;
                let p3 = *last_point + direction * 3.0;
                point_list.push(p1);
                point_list.push(p2);
                point_list.push(p3);

                // spawn new food at random position
                spawn_food_count(&mut commands, &asset_server, 1);
            }
        }
    }
}

pub fn cleanup_foods(mut commands: Commands, query: Query<Entity, With<Food>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
