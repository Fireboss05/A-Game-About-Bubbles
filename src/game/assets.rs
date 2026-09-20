use bevy::prelude::*;
use bevy::{
    asset::{
        io::Reader,
        AssetLoader,
        LoadContext,
        Asset,
    }, 
    reflect::TypePath};

use crate::game::components::{ChoiceDefinition, ChoiceDefinitionsFile, QuestionDefinition, QuestionDefinitionsFile};

use thiserror::Error;


#[derive(Asset, TypePath, Debug)]
pub struct QuestionDefinitions {
    pub questions: Vec<QuestionDefinition>,
}

#[derive(Asset, TypePath, Debug)]
pub struct ChoiceDefinitions {
    pub choices: Vec<ChoiceDefinition>,
}

#[derive(Default, TypePath)]
pub struct QuestionDefinitionsLoader;

#[derive(Default, TypePath)]
pub struct ChoiceDefinitionsLoader;

#[derive(Debug, Error)]
pub enum QuestionDefinitionsLoaderError {
    #[error("Failed to read questions.json: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to parse questions.json: {0}")]
    Json(#[from] serde_json::Error),
}

#[derive(Debug, Error)]
pub enum ChoiceDefinitionsLoaderError {
    #[error("Failed to read choices.json: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to parse choices.json: {0}")]
    Json(#[from] serde_json::Error),
}

impl AssetLoader for QuestionDefinitionsLoader {
    type Asset = QuestionDefinitions;
    type Settings = ();
    type Error = QuestionDefinitionsLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();

        reader.read_to_end(&mut bytes).await?;

        let file: QuestionDefinitionsFile =
            serde_json::from_slice(&bytes)?;

        Ok(QuestionDefinitions {
           questions: file.questions,
        })
    }

    fn extensions(&self) -> &[&str] {
        &["json"]
    }
}

impl AssetLoader for ChoiceDefinitionsLoader {
    type Asset = ChoiceDefinitions;
    type Settings = ();
    type Error = ChoiceDefinitionsLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();

        reader.read_to_end(&mut bytes).await?;

        let file: ChoiceDefinitionsFile =
            serde_json::from_slice(&bytes)?;

        Ok(ChoiceDefinitions {
            choices: file.choices,
        })
    }

    fn extensions(&self) -> &[&str] {
        &["json"]
    }
}