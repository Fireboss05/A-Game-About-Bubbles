use bevy::prelude::*;

pub const NORMAL_SEARCH_BAR_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);

pub fn search_bar_menu_button() -> impl Scene {
    bsn! {
        Node {
            width: px(800),
            height: px(80),
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Center,
            padding: UiRect::horizontal(px(20)),
        }
    }
}

pub fn search_bar_menu_item() -> impl Scene {
    bsn! {
        Node {
            width: px(800),
            height: px(40),
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Center,
            padding: UiRect::horizontal(px(20)),
        }
        BackgroundColor(NORMAL_SEARCH_BAR_COLOR)
    }
}

