use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::srgb(0.35, 0.75, 0.35);

pub fn button_style() -> Node {
    Node {
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        height: Val::Px(80.0),
        width: Val::Px(200.0),
        ..default()
    }
}

pub fn get_button_text(asset_server: &Res<AssetServer>) -> impl Bundle {
    (
        TextFont {
            font: FontSource::Handle(asset_server.load("fonts/DejaVuSans-Bold.ttf")),
            font_size: FontSize::Px(32.0),
            ..default()
        },
        TextColor(Color::WHITE),
    )
}