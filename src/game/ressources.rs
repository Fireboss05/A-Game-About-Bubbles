use bevy::{asset::Handle, prelude::Resource};

use crate::game::assets::{ChoiceDefinitions, QuestionDefinitions};

#[derive(Resource, Debug)]
pub struct GameData {
    pub questions: Handle<QuestionDefinitions>,
    pub choices: Handle<ChoiceDefinitions>,
}
