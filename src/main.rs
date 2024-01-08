use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

// x coordinates
const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
// y coordinates
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;

const SNAKE_HEAD_SIZE: Vec3 = Vec3::new(20., 20., 0.);
const SNAKE_STARTING_POSITION: Vec3 = Vec3::new(0., -50., 1.);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct SnakeHead;

#[derive(Component)]
struct SnakeBody(Vec3);

#[derive(Bundle)]
struct SnakeBundle {
    head: SnakePart,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Snake
    let default_snake_size = 20;

    // Center of the screen
    let head_x = (TOP_WALL - BOTTOM_WALL) / 2.;
    let head_y = (RIGHT_WALL - LEFT_WALL) / 2.;

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::default().into()).into(),
            material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
            transform: Transform::from_translation(SNAKE_STARTING_POSITION)
                .with_scale(SNAKE_HEAD_SIZE),
            ..Default::default()
        },
        SnakeHead,
    ));
}
