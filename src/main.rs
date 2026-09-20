

use bevy::feathers::theme::UiTheme;
use bevy::state::{
    state::States,
    app::AppExtStates,
    condition::in_state
};

use bevy::{DefaultPlugins, app::prelude::*};
use bevy::feathers::FeathersPlugins;

use bevy::ecs::schedule::IntoScheduleConfigs;

mod light_theme;
use light_theme::create_light_theme;

mod loading;
use loading::systems::wait::*;

mod main_page;
use main_page::MainPagePlugin;

mod game;
use game::GamePlugin;

mod systems;
use systems::*;

use crate::game::GameDataPlugin;
use crate::main_page::MainPageDataPlugin;
use crate::main_page::ressources::GameSession;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.init_state::<AppState>();
    app.init_resource::<GameSession>();

    app.add_plugins((MainPagePlugin, MainPageDataPlugin, GamePlugin, GameDataPlugin, FeathersPlugins));
    app.insert_resource(UiTheme(create_light_theme()));
    app.add_systems(Startup, spawn_camera);
    app.add_systems(Startup, (load_main_page_data, load_game_data));
    
    app.add_systems(Update, wait_for_game_data.run_if(in_state(AppState::Loading)),);
    app.add_systems(Update, transition_to_main_page_state);
    app.add_systems(Update, transition_to_game_state);

    app.run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Loading,
    MainPage,
    Game,
    GameOver,
}