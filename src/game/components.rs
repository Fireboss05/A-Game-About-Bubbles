use bevy::{ecs::message::Message, prelude::Component};
use serde::Deserialize;

#[derive(Component, Clone, Copy, Default)]
pub struct Browser {}

// ======= Search Bar ========
#[derive(Component, Clone, Copy, Default)]
pub struct SearchBar {
    pub question_id: u32,
}

#[derive(Component, Clone, Copy, Default)]
pub struct ChoiceButton {
    pub choice_id: u32,
}

#[derive(Message, Clone, Copy, Default)]
pub struct ChoiceSelected {
    pub choice_id: u32,
}



#[derive(Deserialize, Debug, Clone)]
pub struct QuestionDefinition{
    pub id: u32,
    pub character_name: String,
    pub text: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ChoiceDefinition{
    pub id: u32,
    pub question_id: u32,
    pub text: String,
    pub value: u32
}

#[derive(Deserialize)]
pub struct QuestionDefinitionsFile {
    pub questions: Vec<QuestionDefinition>,
}

#[derive(Deserialize)]
pub struct ChoiceDefinitionsFile {
    pub choices: Vec<ChoiceDefinition>,
}

#[derive(Component, Clone, Copy, Default)]
pub struct ChoiceMenuItem {
    pub id: u32
}

// ======= AIResult ========
// #[derive(Component, Clone, Copy, Default)]
// pub struct AIResult {}

// ======= Website ========
// #[derive(Component, Clone, Copy, Default)]
// pub struct Website {}


