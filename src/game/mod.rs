use bevy::prelude::*;

mod components;
mod styles;
mod systems;

use systems::layout::*;

use crate::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin{
    fn build(&self, app: &mut App){
        app.add_systems(OnEnter(AppState::Game), spawn_game);
        app.add_systems(OnExit(AppState::Game), despawn_game);
        println!("game_plugin built");
    }

}
