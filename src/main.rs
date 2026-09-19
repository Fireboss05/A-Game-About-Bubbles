use bevy::state::app::AppExtStates;
use bevy::state::state::States;
use bevy::{DefaultPlugins, app::prelude::*};

mod main_page;
use main_page::MainPagePlugin;

mod game;
use game::GamePlugin;

mod systems;
use  systems::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.init_state::<AppState>();
    app.add_plugins((MainPagePlugin, GamePlugin));
    app.add_systems(Startup, spawn_camera);
    app.add_systems(Update, transition_to_main_page_state);
    app.add_systems(Update, transition_to_game_state);

    app.run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainPage,
    Game,
    GameOver,
}