use bevy::prelude::*;

pub mod components;
pub mod styles;
pub mod systems;
pub mod elements;
pub mod ressources;
pub mod assets;

use systems::layout::*;

use crate::{AppState, game::{assets::{ChoiceDefinitions, ChoiceDefinitionsLoader, QuestionDefinitions, QuestionDefinitionsLoader}, components::ChoiceDefinition}};

pub struct GamePlugin;

impl Plugin for GamePlugin{
    fn build(&self, app: &mut App){
        app.add_systems(OnEnter(AppState::Game), spawn_game);
        app.add_systems(OnExit(AppState::Game), despawn_game);
        println!("game_plugin built");
    }

}

pub struct GameDataPlugin;

impl Plugin for GameDataPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<QuestionDefinitions>()
            .register_asset_loader(QuestionDefinitionsLoader);
        app.init_asset::<ChoiceDefinitions>()
            .register_asset_loader(ChoiceDefinitionsLoader);
    }
}