use bevy::prelude::*;

use crate::{game::{
    assets::{ChoiceDefinitions, QuestionDefinitions}, components::{
        // AIResult, 
        Browser, ChoiceDefinition, QuestionDefinition, 
        // SearchBar, Website
    }, elements::search_bar::layout::*, ressources::GameData
}, main_page::ressources::GameSession};

use rand::seq::SliceRandom;

pub fn spawn_game(mut commands: Commands, game_data: Res<GameData>, 
    questions: Res<Assets<QuestionDefinitions>>, choices: Res<Assets<ChoiceDefinitions>>,
    game_session: Res<GameSession> ) {
    commands.spawn_scene(build_game(game_data, questions, choices, game_session));
    println!("game Spawned");
}


pub fn despawn_game(mut commands: Commands, game_query: Query<Entity, With<Browser>>) {
    if let Ok(game_entity) = game_query.single() {
        commands.entity(game_entity).despawn();
        println!("game despawned");
    }
    
}

pub fn build_game(game_data: Res<GameData>, questions: Res<Assets<QuestionDefinitions>>, choices: Res<Assets<ChoiceDefinitions>>,
    game_session: Res<GameSession>
) -> impl Scene {
    let questions_data = questions
        .get(&game_data.questions)
        .expect("Questions not loaded");

    let selected_question :&QuestionDefinition = questions_data
        .questions
        .iter()
        .find(|question| {
            question.character_name == game_session.character_name.clone().expect("No character selected")
                && question.id == game_session.question_number.expect("No question selected")
        })
        .expect("Question not found");

    let choices_data = choices
        .get(&game_data.choices)
        .expect("Choices not loaded");

    let mut last_answer_value = game_session.last_score;
    if last_answer_value < 1{
        last_answer_value = 2;
        println!("last_answer < 1")
    }
    if game_session.last_question_max == 3 && selected_question.max_score == 9{
        last_answer_value *= 3;
        println!("last_question_max == 3 && selected_question.max_score == 9")
    }
    println!("last_answer_value: {}" , last_answer_value);
    println!("game_session.last_question_max {}", game_session.last_question_max );
    println!("game_session.last_question_min {}", game_session.last_question_min );
    let mut selected_choices: Vec<ChoiceDefinition> = choices_data
        .choices
        .iter()
        .filter(|choice| {
            println!("choice.value {}", choice.value);
            choice.question_id == selected_question.id
            && (choice.value.abs_diff(last_answer_value) <= 1 || (
            choice.value.abs_diff(last_answer_value) <= 2 && (last_answer_value == game_session.last_question_max 
                || last_answer_value == game_session.last_question_min)))
        })
        .cloned()
        .collect();
    let mut rng = rand::rng();
    selected_choices.shuffle(&mut rng);
    bsn! {
        Node{
            // box_sizing: BoxSizing::BorderBox,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            padding: UiRect::all(Val::Px(20.0))
        }
        Browser {}
        BackgroundColor(Color::srgb(0.968627451, 0.8705882353, 0.6705882353))
        Children [
            //Searche Bar / Select bar
            search_bar(selected_question.clone(), selected_choices.clone())
            
            //AI Results
            // bsn! {
            // },
            //internet results
            // bsn! {
            // },
        ]
    }
}