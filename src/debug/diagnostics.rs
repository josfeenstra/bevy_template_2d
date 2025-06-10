use bevy::{
    diagnostic::{
        DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
    },
    prelude::*,
    render::diagnostic::RenderDiagnosticsPlugin,
};
use bevy_console::ConsoleSet;

#[derive(Component)]
struct DiagnosticsDisplay;

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct EntityCountText;

#[derive(Component)]
struct MemoryText;

#[derive(Component)]
struct RenderText;

// add some state to the app to toggle the diagnostics
#[derive(States, Default, Clone, Eq, PartialEq, Hash, Debug)]
#[states(scoped_entities)]
enum DiagnosticsState {
    Enabled,
    #[default]
    Disabled,
}

pub(super) fn plugin(app: &mut App) {
    // add bevy diagnostics plugins
    // DiagnosticsPlugin::default() is added by default in main.rs
    app.init_state::<DiagnosticsState>();
    app.add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(EntityCountDiagnosticsPlugin::default())
        .add_plugins(SystemInformationDiagnosticsPlugin::default())
        .add_plugins(RenderDiagnosticsPlugin::default());

    // add a way to print it to the screen
    app.add_systems(
        OnEnter(DiagnosticsState::Enabled),
        DiagnosticsUi::spawn_driver.in_set(ConsoleSet::ConsoleUI),
    )
    .add_systems(
        Update,
        DiagnosticsUi::update_driver
            .in_set(ConsoleSet::ConsoleUI)
            .run_if(in_state(DiagnosticsState::Enabled)),
    )
    .add_systems(Update, DiagnosticsUi::toggle.in_set(ConsoleSet::ConsoleUI));
}

struct DiagnosticsUi;

impl DiagnosticsUi {
    pub fn toggle(
        state: Res<State<DiagnosticsState>>,
        input: Res<ButtonInput<KeyCode>>,
        mut next_game_state: ResMut<NextState<DiagnosticsState>>,
    ) {
        if input.just_pressed(KeyCode::F2) {
            let new_state = if *state == DiagnosticsState::Enabled {
                DiagnosticsState::Disabled
            } else {
                DiagnosticsState::Enabled
            };

            next_game_state.set(new_state);
        }
    }

    pub fn spawn_driver(mut c: Commands) {
        // Create the main UI container in top-left
        c.spawn((
            StateScoped(DiagnosticsState::Enabled),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(5.0),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            DiagnosticsDisplay,
        ))
        .with_children(|parent| {
            // FPS Text
            parent.spawn((
                Text::new("FPS: --"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.2, 1.0, 0.2)), // Green
                FpsText,
            ));

            // Entity Count Text
            parent.spawn((
                Text::new("Entities: --"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.2, 0.8, 1.0)), // Cyan
                EntityCountText,
            ));

            // Memory Text
            parent.spawn((
                Text::new("Memory: --"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.8, 0.2)), // Yellow
                MemoryText,
            ));
            // Render Text
            parent.spawn((
                Text::new("Render: --"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.8, 0.2)), // Yellow
                RenderText,
            ));
        });
    }

    pub fn update_driver(
        diagnostics: Res<DiagnosticsStore>,
        mut fps_query: Query<
            &mut Text,
            (With<FpsText>, Without<EntityCountText>, Without<MemoryText>),
        >,
        mut entity_query: Query<
            &mut Text,
            (With<EntityCountText>, Without<FpsText>, Without<MemoryText>),
        >,
        mut memory_query: Query<
            &mut Text,
            (With<MemoryText>, Without<FpsText>, Without<EntityCountText>),
        >,
        mut render_query: Query<
            &mut Text,
            (
                With<RenderText>,
                Without<FpsText>,
                Without<EntityCountText>,
                Without<MemoryText>,
            ),
        >,
    ) {
        // Update FPS
        if let Ok(mut text) = fps_query.single_mut() {
            if let Some(fps_diagnostic) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(fps_smoothed) = fps_diagnostic.smoothed() {
                    **text = format!("FPS: {:.1}", fps_smoothed);
                }
            }
        }

        // Update Entity Count
        if let Ok(mut text) = entity_query.single_mut() {
            if let Some(entity_diagnostic) =
                diagnostics.get(&EntityCountDiagnosticsPlugin::ENTITY_COUNT)
            {
                if let Some(entity_count) = entity_diagnostic.smoothed() {
                    **text = format!("Entities: {:.0}", entity_count);
                }
            }
        }

        // Update Memory Usage (from system diagnostics)
        if let Ok(mut text) = memory_query.single_mut() {
            if let Some(memory_diagnostic) =
                diagnostics.get(&SystemInformationDiagnosticsPlugin::PROCESS_MEM_USAGE)
            {
                if let Some(total_memory) = memory_diagnostic.value() {
                    let memory_gb = total_memory / (1024.0 * 1024.0 * 1024.0);
                    **text = format!("Total Memory: {:.1} GB", memory_gb);
                }
            } else {
                **text = "Memory: N/A".to_string();
            }
        }

        // Update Render Text - access render diagnostics from DiagnosticsStore
        if let Ok(mut _text) = render_query.single_mut() {
            // // Try to get render-related diagnostics from RenderDiagnosticsPlugin
            // // The exact diagnostic key depends on what's available
            // if let Some(render_diagnostic) = diagnostics
            //     .iter()
            //     .find(|diag| diag.name().contains("render") || diag.name().contains("gpu"))
            // {
            //     if let Some(render_time) = render_diagnostic.smoothed() {
            //         **text = format!("Render: {:.2} ms", render_time);
            //     } else {
            //         **text = "Render: N/A".to_string();
            //     }
            // } else {
            //     **text = "Render: No Data".to_string();
            // }
        }
    }
}
