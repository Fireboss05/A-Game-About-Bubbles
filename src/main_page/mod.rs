use bevy::prelude::*;

mod components;
mod styles;
mod systems;

use systems::layout::*;

use crate::AppState;

pub struct MainPagePlugin;

impl Plugin for MainPagePlugin{
    fn build(&self, app: &mut App){
        app.add_systems(OnEnter(AppState::MainPage), spawn_main_page);
        app.add_systems(OnExit(AppState::MainPage), despawn_main_page);
        println!("main_page_plugin built");
    }

}
