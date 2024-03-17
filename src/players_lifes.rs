use bevy::prelude::*;

use crate::app_state::AppState;

#[derive(Component)]
pub struct PlayerLifeUiRoot;

#[derive(Component)]
pub struct PlayerLifeUiText;

#[derive(Resource)]
pub struct PlayersLifes(pub u32);

impl PlayersLifes {
    pub fn new() -> Self {
        Self(1)
    }
}

pub struct PlayersLifesPlugin;

impl Plugin for PlayersLifesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_players_lifes)
            .add_systems(OnExit(AppState::InGame), cleanup_players_lifes_ui)
            .add_systems(
                Update,
                (update_players_lifes_ui, game_over).run_if(in_state(AppState::InGame)),
            );
    }
}

pub fn setup_players_lifes(mut commands: Commands) {
    let root = commands
        .spawn((
            PlayerLifeUiRoot,
            NodeBundle {
                background_color: BackgroundColor(Color::BLACK.with_a(0.5)),
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(10.0),
                    top: Val::Px(10.0),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id();

    let text = commands
        .spawn((
            PlayerLifeUiText,
            TextBundle {
                text: Text::from_sections([
                    TextSection {
                        value: "LIFES: ".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    },
                    TextSection {
                        value: " N/A".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    },
                ]),
                ..Default::default()
            },
        ))
        .id();

    commands.entity(root).push_children(&[text]);

    commands.insert_resource(PlayersLifes::new());
}

pub fn update_players_lifes_ui(
    players_lifes: Res<PlayersLifes>,
    mut query: Query<&mut Text, With<PlayerLifeUiText>>,
) {
    for mut text in &mut query {
        text.sections[1].value = format!(" {:>2.0}", players_lifes.0);
    }
}

pub fn game_over(mut next_state: ResMut<NextState<AppState>>, players_lifes: Res<PlayersLifes>) {
    if players_lifes.0 == 0 {
        next_state.set(AppState::GameOver);
    }
}

pub fn cleanup_players_lifes_ui(
    mut commands: Commands,
    query: Query<Entity, With<PlayerLifeUiRoot>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
