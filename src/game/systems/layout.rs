use bevy::prelude::*;

use crate::game::components::Game;

pub fn spawn_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_game(&mut commands, &asset_server);
    println!("game Spawned");
}

pub fn despawn_game(mut commands: Commands, game_query: Query<Entity, With<Game>>) {
    if let Ok(game_entity) = game_query.single() {
        commands.entity(game_entity).despawn();
        println!("main_game despawned");
    }
    
}

pub fn build_game(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let game_entity: Entity = commands.spawn((Node{
            // box_sizing: BoxSizing::BorderBox,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        }, 
        Game {},
        BackgroundColor(Color::srgb(0.1, 0.25, 0.1))
    )).id();

    game_entity //return
}