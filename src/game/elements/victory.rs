use bevy::prelude::*;

#[derive(Component, Default, Clone)]
pub struct VictoryScreen;

pub fn build_victory_screen() -> impl Scene {
    bsn! {
        VictoryScreen

        Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: px(20),
        }

        BackgroundColor(Color::srgb(0.9, 1.0, 0.9))

        Children[
            (
                Text("VICTORY !")
                TextFont {
                    font_size: FontSize::Px(60.0)
                }
                TextColor(Color::BLACK)
            ),
            (
                Text("Well done, you managed to leave your social bubble")
                TextFont {
                    font_size: FontSize::Px(28.0)
                }
                TextColor(Color::BLACK)
            )
        ]
    }
}

pub fn spawn_victory_screen(mut commands: Commands) {
    commands.spawn_scene(build_victory_screen());
    println!("Victory screen spawned");
}

pub fn despawn_victory_screen(
    mut commands: Commands,
    query: Query<Entity, With<VictoryScreen>>,
) {
    if let Ok(entity) = query.single() {
        commands.entity(entity).despawn();
        println!("Victory screen despawned");
    }
}