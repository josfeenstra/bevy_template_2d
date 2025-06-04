use bevy::{
    ecs::system::SystemParam,
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

use crate::asset_tracking::LoadResource;

#[derive(SystemParam)]
pub struct Store<'w> {
    pub assets: Res<'w, DemoAssets>,
    pub texture_atlas_layouts: ResMut<'w, Assets<TextureAtlasLayout>>,
}

impl<'w> Store<'w> {
    pub(super) fn plugin(app: &mut App) {
        app.register_type::<DemoAssets>();
        app.load_resource::<DemoAssets>();
    }

    pub fn is_ready(
        assets: Option<Res<DemoAssets>>,
        texture_atlas: Option<Res<Assets<TextureAtlasLayout>>>,
    ) -> bool {
        assets.is_some() && texture_atlas.is_some()
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct DemoAssets {
    #[dependency]
    pub ducky: Handle<Image>,
    #[dependency]
    pub steps: Vec<Handle<AudioSource>>,
    #[dependency]
    pub music: Handle<AudioSource>,
}

impl FromWorld for DemoAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            ducky: assets.load_with_settings(
                "images/ducky.png",
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve pixel art style.
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            steps: vec![
                assets.load("audio/sound_effects/step1.ogg"),
                assets.load("audio/sound_effects/step2.ogg"),
                assets.load("audio/sound_effects/step3.ogg"),
                assets.load("audio/sound_effects/step4.ogg"),
            ],
            music: assets.load("audio/music/Fluffing A Duck.ogg"),
        }
    }
}
