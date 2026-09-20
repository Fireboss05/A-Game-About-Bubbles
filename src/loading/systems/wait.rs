use bevy::prelude::*;

use crate::{AppState, game::{assets::{ChoiceDefinitions, QuestionDefinitions}, ressources::GameData}, main_page::{assets::CharacterDefinitions, ressources::MainPageData}};

pub fn wait_for_game_data(
    main_page_data: Res<MainPageData>,
    characters: Res<Assets<CharacterDefinitions>>,
    game_data: Res<GameData>,
    questions: Res<Assets<QuestionDefinitions>>,
    choices: Res<Assets<ChoiceDefinitions>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if characters.get(&main_page_data.characters).is_some()
    && questions.get(&game_data.questions).is_some()
    && choices.get(&game_data.choices).is_some() {
        next_state.set(AppState::MainPage);
    }
}