use bevy::prelude::*;

pub fn spawn_main_page(mut commands: Commands, asset_server: Res<AssetServer>) {
    let main_page_entity = build_main_page(&mut commands, &asset_server);
}

pub fn despawn_main_page() {

}

pub fn build_main_page(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let main_page_entity: Entity = commands.spawn(NodeBundle{
        background_color: Color::RED.into(),
        ..default()
    }).id();

    main_page_entity //return
}