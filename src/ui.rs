use bevy::prelude::*;

use crate::app_state::AppState;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), create_ui)
            .add_systems(OnExit(AppState::MainMenu), cleanup_ui)
            .add_systems(
                Update,
                button_interaction_system.run_if(in_state(AppState::MainMenu)),
            );
    }
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);

#[derive(Component)]
struct MainMenuUi;

/// Create a simple UI with Play Button that set AppState to InGame
fn create_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn((
            MainMenuUi,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
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
                })
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
        });
}

type BtnInteractionQuery<'world, 'state, 'a, 'b, 'c, 'd> = Query<
    'world,
    'state,
    (
        &'a Interaction,
        &'b mut BackgroundColor,
        &'c mut BorderColor,
        &'d Children,
    ),
    (Changed<Interaction>, With<Button>),
>;

fn button_interaction_system(
    mut interaction_query: BtnInteractionQuery,
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

fn cleanup_ui(mut commands: Commands, query: Query<Entity, With<MainMenuUi>>) {
    for entity in &mut query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
