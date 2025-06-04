//! Spawn the main level.

use bevy::prelude::*;

use crate::{
    audio::music,
    demo::{player::Player, store::Store},
    screens::Screen,
};

#[derive(Component)]
pub struct Level;

impl Level {
    /// A system that spawns the main level.
    pub fn level(store: &mut Store) -> impl Bundle {
        let music = music(store.assets.music.clone());
        (
            Level,
            Name::new("Level"),
            Transform::default(),
            Visibility::default(),
            StateScoped(Screen::Gameplay),
            children![
                Player::player(store, 400.0),
                (Name::new("Gameplay Music"), music),
            ],
        )
    }
}
