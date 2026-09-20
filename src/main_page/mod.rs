use bevy::prelude::*;

pub mod components;
pub mod ressources;
pub mod styles;
pub mod systems;
pub mod assets;

use systems::layout::*;
use systems::interactions::*;

use crate::AppState;
use crate::main_page::assets::{
    CharacterDefinitionsLoader,
    CharacterDefinitions
};

pub struct MainPagePlugin;

impl Plugin for MainPagePlugin{
    fn build(&self, app: &mut App){
        app.add_systems(OnEnter(AppState::MainPage), spawn_main_page);
        app.add_systems(OnExit(AppState::MainPage), despawn_main_page);
        app.add_systems(Update,
            (interact_with_character_button, interact_with_quit_button)
            .run_if(in_state(AppState::MainPage))
        );
    }

}

pub struct GameDataPlugin;

impl Plugin for GameDataPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<CharacterDefinitions>()
            .register_asset_loader(CharacterDefinitionsLoader);
    }
}
