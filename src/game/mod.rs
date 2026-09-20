use bevy::prelude::*;

pub mod assets;
pub mod components;
pub mod elements;
pub mod ressources;
pub mod styles;
pub mod systems;

use systems::layout::*;
use systems::interactions::interact_with_choice_button;

use crate::{
    AppState,
    game::{
        assets::{
            ChoiceDefinitions, ChoiceDefinitionsLoader, QuestionDefinitions,
            QuestionDefinitionsLoader,
        }
    },
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_game);
        app.add_systems(OnExit(AppState::Game), despawn_game);
        app.add_systems(
            Update,
            (interact_with_choice_button).run_if(in_state(AppState::Game)),
        );
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
