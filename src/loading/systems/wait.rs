use bevy::prelude::*;

use crate::{AppState, main_page::{assets::CharacterDefinitions, ressources::GameData}};

pub fn wait_for_game_data(
    game_data: Res<GameData>,
    characters: Res<Assets<CharacterDefinitions>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if characters.get(&game_data.characters).is_some() {
        next_state.set(AppState::MainPage);
    }
}