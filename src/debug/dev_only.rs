use crate::screens::Screen;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;

use bevy::dev_tools::states::log_transitions;
use bevy::ui::UiDebugOptions;

const TOGGLE_KEY: KeyCode = KeyCode::F1;

pub fn plugin(app: &mut App) {
    // Log `Screen` state transitions.
    // Toggle the debug overlay for UI.
    app.add_systems(
        Update,
        (
            log_transitions::<Screen>,
            toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY)),
        ),
    );
}

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}
