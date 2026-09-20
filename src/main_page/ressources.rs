use bevy::{asset::Handle, prelude::Resource};

use crate::main_page::assets::CharacterDefinitions;

#[derive(Resource, Debug)]
pub struct MainPageData {
    pub characters: Handle<CharacterDefinitions>,
}
