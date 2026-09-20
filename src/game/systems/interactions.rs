use bevy::prelude::*;

use bevy::{
    asset::Assets, ecs::{
        system::ResMut,
    }, state::state::NextState,
};

use crate::game::components::ChoiceSelected;
use crate::{
    AppState, game::{
        assets::{ChoiceDefinitions, QuestionDefinitions}, components::ChoiceDefinition, ressources::GameData,
    }, main_page::ressources::GameSession,
};

pub fn interact_with_choice_button(
    mut messages: MessageReader<ChoiceSelected>,
    game_data: Res<GameData>,
    questions: Res<Assets<QuestionDefinitions>>,
    choices: Res<Assets<ChoiceDefinitions>>,
    mut app_state_next_state: ResMut<NextState<AppState>>, //to go to winning ?
    mut game_session: ResMut<GameSession>,
) {
    for choice_message in messages.read() {
        println!("Choice selected: {}", choice_message.choice_id);

        let choice_selected: ChoiceDefinition = choices
            .get(&game_data.choices)
            .expect("Choices not loaded")
            .choices
            .iter()
            .find(|choice| choice.id == choice_message.choice_id)
            .expect("No choice found for selected choice").clone();
        
        game_session.last_score = choice_selected.value;
        game_session.total_score += 9 - choice_selected.value;
        
        let character_name = game_session
            .character_name
            .as_deref()
            .expect("No character selected");

        let current_question = game_session
            .question_number
            .expect("No current question");

        if choice_selected.value != 9{
            let next_question = questions
                .get(&game_data.questions)
                .expect("Questions not loaded")
                .questions
                .iter()
                .filter(|question| {
                    question.character_name == character_name
                        && question.id > current_question
                })
                .min_by_key(|question| question.id);

            match next_question {
                Some(question) => {
                    game_session.question_number = Some(question.id);
                    app_state_next_state.set(AppState::Game);
                }

                None => {
                    // 
                    // app_state_next_state.set(AppState::Lost);
                }
            }

                //should reload browser since the question changes
        }else{
            // app_state_next_state.set(AppState::Won);
        }
    }
}
