//! Spawn the main level.

use bevy::prelude::*;

use crate::{
    asset_tracking::LoadResource,
    audio::music,
    demo::player::{Player, PlayerAssets},
    screens::Screen,
};

#[derive(Component)]
pub struct Level;

impl Level {
    pub(super) fn plugin(app: &mut App) {
        app.register_type::<LevelAssets>();
        app.load_resource::<LevelAssets>();
    }

    /// A system that spawns the main level.
    pub fn bundle(
        mut commands: Commands,
        level_assets: Res<LevelAssets>,
        player_assets: Res<PlayerAssets>,
        mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    ) -> impl Bundle {
        (
            Level,
            Name::new("Level"),
            Transform::default(),
            Visibility::default(),
            StateScoped(Screen::Gameplay),
            children![
                Player::bundle(400.0, &player_assets, &mut texture_atlas_layouts),
                (
                    Name::new("Gameplay Music"),
                    music(level_assets.music.clone())
                )
            ],
        )
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    music: Handle<AudioSource>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/Fluffing A Duck.ogg"),
        }
    }
}
