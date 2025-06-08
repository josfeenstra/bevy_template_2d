use bevy::prelude::*;

pub mod console;
pub mod diagnostics;

#[cfg(feature = "dev")]
pub mod dev_only;

pub fn plugin(app: &mut App) {
    app.add_plugins(console::plugin);
    app.add_plugins(diagnostics::plugin);
    #[cfg(feature = "dev")]
    app.add_plugins(dev_only::plugin);
}
