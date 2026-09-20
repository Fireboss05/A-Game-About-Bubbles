use bevy::prelude::*;

use crate::main_page::{assets::CharacterDefinitions, components::{CharacterButton, MainPage, QuitButton}, ressources::GameData, styles::{NORMAL_BUTTON_COLOR, button_style, get_button_text}};

pub fn spawn_main_page(mut commands: Commands, asset_server: Res<AssetServer>,
    game_data: Res<GameData>, characters: Res<Assets<CharacterDefinitions>>) {
    build_main_page(&mut commands, &asset_server, game_data, characters);
    println!("main_page Spawned");
}

pub fn despawn_main_page(mut commands: Commands, main_page_query: Query<Entity, With<MainPage>>) {
    if let Ok(main_page_entity) = main_page_query.single() {
        commands.entity(main_page_entity).despawn();
        println!("main_page despawned");
    }
    
}

pub fn build_main_page(commands: &mut Commands, asset_server: &Res<AssetServer>, 
    game_data: Res<GameData>, characters: Res<Assets<CharacterDefinitions>>) -> Entity {
    let main_page_entity: Entity = commands.spawn((Node{
            // box_sizing: BoxSizing::BorderBox,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        }, 
        MainPage {},
        BackgroundColor(Color::srgb(1.0, 1.0, 1.0))
    ))
    .with_children(|parent|{
        // Title
        parent.spawn((

        ));

        //Character Button (stories)
        if let Some(data) = characters.get(&game_data.characters) {
            for character in &data.characters {
                parent.spawn((
                    Button,
                    button_style(),
                    BackgroundColor(NORMAL_BUTTON_COLOR),
                    CharacterButton{}
                ))
                .with_children(|parent|{
                    parent.spawn((
                        Text::new(character.name.clone()),
                        get_button_text(asset_server)
                    ));
                });
                println!("{}", character.name);
            }
        }

        //Quit Button
        parent.spawn((
            Button,
            button_style(),
            BackgroundColor(NORMAL_BUTTON_COLOR),
            QuitButton{}
        ))
        .with_children(|parent|{
            parent.spawn((
                Text::new("Quit"),
                get_button_text(asset_server)
            ));
        });
    })
    .id();

    main_page_entity //return
}