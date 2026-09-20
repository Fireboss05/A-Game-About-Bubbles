use bevy::prelude::Component;
use serde::Deserialize;

#[derive(Component)]
pub struct MainPage {}

#[derive(Deserialize, Debug)]
pub struct CharacterDefinition{
    pub name: String,
    pub description: String
}

#[derive(Deserialize)]
pub struct CharacterDefinitionsFile {
    pub characters: Vec<CharacterDefinition>,
}

#[derive(Component)]
pub struct CharacterButton {}

#[derive(Component)]
pub struct QuitButton {}