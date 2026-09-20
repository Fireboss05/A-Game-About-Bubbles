use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::AppState;
use crate::game::assets::{ChoiceDefinitions, QuestionDefinitions};
use crate::game::ressources::GameData;
use crate::main_page::assets::CharacterDefinitions;
use crate::main_page::ressources::MainPageData;

pub fn transition_to_main_page_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if *app_state.get() != AppState::Loading && keyboard_input.just_pressed(KeyCode::Space) {
        if *app_state.get() != AppState::MainPage {
            app_state_next_state.set(AppState::MainPage);
            println!("Entered AppState::MainMenu");
        }
    }
}

pub fn transition_to_game_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if *app_state.get() != AppState::Loading && keyboard_input.just_pressed(KeyCode::KeyG) {
        if *app_state.get() != AppState::Game {
            app_state_next_state.set(AppState::Game);
            println!("Entered AppState::Game");
        }
    }
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single().unwrap();

    commands.spawn((Camera2d,
        Transform::from_xyz(
            window.width() / 2.0,
            window.height() / 2.0,
            0.0,
        )));
}

pub fn load_main_page_data(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let characters: Handle<CharacterDefinitions> =
        asset_server.load("characters/characters.json");

    commands.insert_resource(MainPageData {
        characters,
    });
}

pub fn load_game_data(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let questions: Handle<QuestionDefinitions> =
        asset_server.load("questions/questions.json");
    let choices: Handle<ChoiceDefinitions> =
        asset_server.load("choices/choices.json");

    commands.insert_resource(GameData {
        questions,
        choices
    });
}