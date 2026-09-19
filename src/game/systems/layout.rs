use bevy::prelude::*;

use crate::game::{
    components::{AIResult, Browser, ChoiceDefinition, QuestionDefinition, SearchBar, Website}, elements::search_bar::layout::*, styles::{}
};


pub fn spawn_game(mut commands: Commands) {
    commands.spawn_scene(build_game());
    println!("game Spawned");
}


pub fn despawn_game(mut commands: Commands, game_query: Query<Entity, With<Browser>>) {
    if let Ok(game_entity) = game_query.single() {
        commands.entity(game_entity).despawn();
        println!("game despawned");
    }
    
}

pub fn build_game() -> impl Scene {
    bsn! {
        Node{
            // box_sizing: BoxSizing::BorderBox,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            padding: UiRect::all(Val::Px(20.0))
        }
        Browser
        BackgroundColor(Color::srgb(0.968627451, 0.8705882353, 0.6705882353))
        Children [
            //Searche Bar / Select bar
            search_bar(&QuestionDefinition{
                id: 1,
                text: "How to"
            }, &[ChoiceDefinition{
                id: 1,
                question_id: 1,
                text: "create my own brand"
            },ChoiceDefinition{
                id: 1,
                question_id: 2,
                text: "take my business to the next level"
            }])
            
            //AI Results
            // bsn! {
            // },
            //internet results
            // bsn! {
            // },
        ]
    }
}