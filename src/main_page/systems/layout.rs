use bevy::prelude::*;

use crate::main_page::components::MainPage;

pub fn spawn_main_page(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_main_page(&mut commands, &asset_server);
    println!("main_page Spawned");
}

pub fn despawn_main_page(mut commands: Commands, main_page_query: Query<Entity, With<MainPage>>) {
    if let Ok(main_page_entity) = main_page_query.single() {
        commands.entity(main_page_entity).despawn();
        println!("main_page despawned");
    }
    
}

pub fn build_main_page(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let main_page_entity: Entity = commands.spawn((Node{
        // box_sizing: BoxSizing::BorderBox,
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        ..default()
    }, 
    MainPage {},
    BackgroundColor(Color::srgb(0.25, 0.1, 0.1))
)).id();

    main_page_entity //return
}