use bevy::{asset::Handle, prelude::Resource};

use crate::main_page::assets::CharacterDefinitions;

#[derive(Resource, Debug)]
pub struct MainPageData {
    pub characters: Handle<CharacterDefinitions>,
}

#[derive(Resource, Default)]
pub struct GameSession {
    pub character_name: Option<String>,
    pub question_number: Option<u32>,
    pub total_score: u32,
    pub last_score: u32,
    pub last_question_min: u32,
    pub last_question_max: u32
}
