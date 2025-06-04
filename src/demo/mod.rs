//! Demo gameplay. All of these modules are only intended for demonstration
//! purposes and should be replaced with your own game logic.
//! Feel free to change the logic found here if you feel like tinkering around
//! to get a feeling for the template.

use bevy::prelude::*;

use crate::demo::{animation::PlayerAnimation, player::Player, store::Store};

mod animation;
pub mod level;
mod movement;
pub mod player;
pub mod store;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        PlayerAnimation::plugin,
        movement::plugin,
        Player::plugin,
        Store::plugin,
    ));
}
