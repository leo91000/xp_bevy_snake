use app_state::AppStatePlugin;
use bevy::prelude::*;
use collision::CollisionPlugin;
use food::FoodPlugin;
use fps_counter::FpsCounterPlugin;
use game::GamePlugin;
use players_lifes::PlayersLifesPlugin;
use stepping::SteppingEguiPlugin;
use ui::UIPlugin;
use ui_game_over::UIGameOverPlugin;

mod app_state;
mod collision;
mod components;
mod consts;
mod food;
mod fps_counter;
mod game;
mod levels;
mod players_lifes;
mod stepping;
mod ui;
mod ui_game_over;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            SteppingEguiPlugin::default()
                .add_schedule(Update)
                .add_schedule(FixedUpdate),
        )
        .add_plugins(FpsCounterPlugin)
        .add_plugins(AppStatePlugin)
        .add_plugins(PlayersLifesPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(CollisionPlugin)
        .add_plugins(UIPlugin)
        .add_plugins(UIGameOverPlugin)
        .add_plugins(FoodPlugin)
        .run();
}
