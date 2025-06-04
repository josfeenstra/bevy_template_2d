//! Player-specific behavior.

use bevy::prelude::*;

use crate::{
    AppSystems, PausableSystems,
    demo::{
        animation::PlayerAnimation,
        movement::{MovementController, ScreenWrap},
        store::Store,
    },
};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

impl Player {
    pub(super) fn plugin(app: &mut App) {
        app.register_type::<Player>();

        // Record directional input as movement controls.
        app.add_systems(
            Update,
            Self::directional_input_driver
                .in_set(AppSystems::RecordInput)
                .in_set(PausableSystems),
        );
    }

    /// The player character.
    pub fn player(store: &mut Store, max_speed: f32) -> impl Bundle {
        // A texture atlas is a way to split a single image into a grid of related images.
        // You can learn more in this example: https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
        let layout =
            TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 2, Some(UVec2::splat(1)), None);
        let texture_atlas_layout = store.texture_atlas_layouts.add(layout);
        let player_animation = PlayerAnimation::new();

        (
            Name::new("Player"),
            Player,
            Sprite {
                image: store.assets.ducky.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: texture_atlas_layout,
                    index: player_animation.get_atlas_index(),
                }),
                ..default()
            },
            Transform::from_scale(Vec2::splat(8.0).extend(1.0)),
            MovementController {
                max_speed,
                ..default()
            },
            ScreenWrap,
            player_animation,
        )
    }

    fn directional_input_driver(
        input: Res<ButtonInput<KeyCode>>,
        mut controller_query: Query<&mut MovementController, With<Player>>,
    ) {
        // Collect directional input.
        let mut intent = Vec2::ZERO;
        if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
            intent.y += 1.0;
        }
        if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
            intent.y -= 1.0;
        }
        if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
            intent.x -= 1.0;
        }
        if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
            intent.x += 1.0;
        }

        // Normalize intent so that diagonal movement is the same speed as horizontal / vertical.
        // This should be omitted if the input comes from an analog stick instead.
        let intent = intent.normalize_or_zero();

        // Apply movement intent to controllers.
        for mut controller in &mut controller_query {
            controller.intent = intent;
        }
    }
}
