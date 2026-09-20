use bevy::prelude::Component;

#[derive(Component, Clone, Copy, Default)]
pub struct Browser {}

// ======= Search Bar ========
#[derive(Component, Clone, Copy, Default)]
pub struct SearchBar {}

pub struct QuestionDefinition<'a>{
    pub id: u32,
    pub text: &'a str
}

pub struct ChoiceDefinition<'a>{
    pub id: u32,
    pub question_id: u32,
    pub text: &'a str
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


