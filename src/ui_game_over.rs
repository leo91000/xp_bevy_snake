use bevy::{app::AppExit, prelude::*};

use crate::app_state::AppState;

pub struct UIGameOverPlugin;

impl Plugin for UIGameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameOver), create_ui)
            .add_systems(OnExit(AppState::GameOver), cleanup_ui)
            .add_systems(
                Update,
                (
                    play_button_interaction_system,
                    exit_button_interaction_system,
                )
                    .run_if(in_state(AppState::GameOver)),
            );
    }
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);

#[derive(Component)]
struct MainMenuUi;

#[derive(Component)]
struct ExitButton;

#[derive(Component)]
struct PlayButton;

/// Create a simple UI with Play Button that set AppState to InGame
fn create_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            MainMenuUi,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            // Game Over txt
            parent.spawn(TextBundle::from_section(
                "GAME OVER",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 50.0,
                    color: Color::WHITE,
                },
            ));

            parent
                .spawn((
                    PlayButton,
                    ButtonBundle {
                        style: Style {
                            margin: UiRect {
                                top: Val::Px(20.0),
                                bottom: Val::Px(10.0),
                                ..Default::default()
                            },
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "PLAY",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });

            parent
                .spawn((
                    ExitButton,
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "EXIT",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

type BtnInteractionQuery<'world, 'state, 'a, 'b, 'c, 'd, T> = Query<
    'world,
    'state,
    (
        &'a Interaction,
        &'b mut BackgroundColor,
        &'c mut BorderColor,
        &'d Children,
    ),
    (Changed<Interaction>, With<Button>, With<T>),
>;

fn play_button_interaction_system(
    mut interaction_query: BtnInteractionQuery<PlayButton>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, _, mut border_color, _) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                border_color.0 = Color::RED;
                next_state.set(AppState::InGame);
            }
            Interaction::Hovered => {
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn exit_button_interaction_system(
    mut interaction_query: BtnInteractionQuery<ExitButton>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, _, mut border_color, _) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                border_color.0 = Color::RED;
                exit.send(AppExit);
            }
            Interaction::Hovered => {
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn cleanup_ui(mut commands: Commands, query: Query<Entity, With<MainMenuUi>>) {
    for entity in &mut query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
