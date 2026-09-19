use bevy::prelude::*;

pub mod components;
pub mod styles;
pub mod systems;

use systems::layout::*;
use systems::interactions::*;

use crate::AppState;

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
