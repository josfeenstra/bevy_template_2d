use bevy::prelude::*;

pub mod console;

#[cfg(feature = "dev")]
pub mod dev_only;

pub fn plugin(app: &mut App) {
    app.add_plugins(console::plugin);
    #[cfg(feature = "dev")]
    app.add_plugins(dev_only::plugin);
}
