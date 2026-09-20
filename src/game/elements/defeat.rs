use bevy::prelude::*;

#[derive(Component, Default, Clone)]
pub struct GameOverScreen;

pub fn build_defeat_screen() -> impl Scene {
    bsn! {
        GameOverScreen

        Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: px(20),
        }

        BackgroundColor(Color::srgb(1.0, 0.9, 0.9))

        Children[
            (
                Text("DEFEAT")
                TextFont {
                    font_size: FontSize::Px(60.0)
                }
                TextColor(Color::BLACK)
            ),
            (
                Text("You Lost... Try to search things you wouldn't usually search for")
                TextFont {
                    font_size: FontSize::Px(28.0)
                }
                TextColor(Color::BLACK)
            )
        ]
    }
}

pub fn spawn_defeat_screen(mut commands: Commands) {
    commands.spawn_scene(build_defeat_screen());
    println!("Game over screen spawned");
}

pub fn despawn_defeat_screen(
    mut commands: Commands,
    query: Query<Entity, With<GameOverScreen>>,
) {
    if let Ok(entity) = query.single() {
        commands.entity(entity).despawn();
        println!("Game over screen despawned");
    }
}