use bevy::prelude::*;

use crate::game::elements::search_bar::styles::*;
use crate::game::components::*;

use bevy::feathers::{
    controls::*,
    theme::ThemedText,
};

pub fn search_bar(question: QuestionDefinition, choices: Vec<ChoiceDefinition>) -> impl Scene {
    let question_text = question.text;
    bsn! {
        @FeathersMenu
        Children[
            (
                @FeathersMenuButton {
                    @caption: bsn! {
                        Text(question_text)
                        ThemedText
                    }
                }
                search_bar_menu_button()
            ),
            (
                @FeathersMenuPopup
                Children[
                    {build_menu_items(choices)}
                ]
            )
        ]
        SearchBar
    }
}

fn build_menu_items(choices: Vec<ChoiceDefinition>) -> impl SceneList {
    let items = choices.iter().map(|choice| {
        let text = choice.clone().text;
        let id = choice.id;
        bsn!{(
                @FeathersMenuItem {
                    @caption: bsn! {
                        Text(text)
                        ChoiceMenuItem{id:id}
                        ThemedText
                    }
                }
                search_bar_menu_item()
            )
        }}
    ).collect::<Vec<_>>();
    bsn_list![{items}]
}