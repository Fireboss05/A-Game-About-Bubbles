use bevy::prelude::*;

use crate::AppState;

use crate::game::assets::QuestionDefinitions;
use crate::game::ressources::GameData;
use crate::main_page::components::*;
use crate::main_page::ressources::GameSession;
use crate::main_page::styles::{HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR};

pub fn interact_with_character_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &CharacterButton),
        (Changed<Interaction>, With<CharacterButton>),
    >,
    game_data: ResMut<GameData>,
    questions: Res<Assets<QuestionDefinitions>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut game_session: ResMut<GameSession>
) {
    if let Ok((interaction, mut background_color, character_button)) = button_query.single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();

                println!("Character selected: {}", character_button.character_name);
                game_session.character_name = Some(character_button.character_name.clone());

                let first_question = questions
                    .get(&game_data.questions)
                    .expect("Questions not loaded")
                    .questions
                    .iter()
                    .filter(|question| {
                        question.character_name == character_button.character_name
                    })
                    .min_by_key(|question| question.id)
                    .expect("No question found for selected character");

                game_session.question_number = Some(first_question.id);

                app_state_next_state.set(AppState::Game);
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn interact_with_quit_button(
    mut app_exit_event_writer: MessageWriter<AppExit>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
) {
    if let Ok((interaction, mut background_color)) = button_query.single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                app_exit_event_writer.write(AppExit::Success);
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}