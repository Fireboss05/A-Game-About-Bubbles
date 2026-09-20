use bevy::prelude::*;
use bevy::{
    asset::{
        io::Reader,
        AssetLoader,
        LoadContext,
        Asset,
    }, 
    reflect::TypePath};

use crate::main_page::components::{
    CharacterDefinition, CharacterDefinitionsFile
};

use thiserror::Error;


#[derive(Asset, TypePath, Debug)]
pub struct CharacterDefinitions {
    pub characters: Vec<CharacterDefinition>,
}

#[derive(Default, TypePath)]
pub struct CharacterDefinitionsLoader;

#[derive(Debug, Error)]
pub enum CharacterDefinitionsLoaderError {
    #[error("Failed to read characters.json: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to parse characters.json: {0}")]
    Json(#[from] serde_json::Error),
}

impl AssetLoader for CharacterDefinitionsLoader {
    type Asset = CharacterDefinitions;
    type Settings = ();
    type Error = CharacterDefinitionsLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();

        reader.read_to_end(&mut bytes).await?;

        let file: CharacterDefinitionsFile =
            serde_json::from_slice(&bytes)?;

        Ok(CharacterDefinitions {
            characters: file.characters,
        })
    }

    fn extensions(&self) -> &[&str] {
        &["json"]
    }
}