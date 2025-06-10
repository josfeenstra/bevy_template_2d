//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_console::{
    AddConsoleCommand, ConsoleCommand, ConsoleCommandEntered, ConsoleConfiguration, ConsolePlugin,
    ConsoleSet, PrintConsoleLine, reply,
};
use bevy_egui::egui::Color32;
use clap::Parser;

const PADDING: f32 = 50.0;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(Console::plugin);
}

/// TODO: disable controls if this is present
/// TODO: rewire print statements to this ingame console
/// TODO: fullscreen or something?
/// TODO: store log to file
#[derive(SystemParam)]
pub struct Console<'w> {
    print_line_event: EventWriter<'w, PrintConsoleLine>,
}
impl<'w> Console<'w> {
    pub fn log(&mut self, line: impl Into<String>) {
        self.print_line_event
            .write(PrintConsoleLine::new(line.into()));
    }
}
impl<'w> Console<'w> {
    pub fn plugin(app: &mut App) {
        app.add_plugins(ConsolePlugin)
            .insert_resource(ConsoleConfiguration {
                keys: vec![KeyCode::Backquote],
                left_pos: PADDING,
                top_pos: PADDING,
                history_size: 20,
                symbol: "> ".to_owned(),
                title_name: "MAGIC WORDS".to_string(),
                collapsible: false,
                resizable: false,
                moveable: false,
                show_title_bar: true,
                background_color: Color32::from_black_alpha(230),
                foreground_color: Color32::LIGHT_GRAY,
                num_suggestions: 4,
                block_mouse: false,
                block_keyboard: true,
                arg_completions: Default::default(),
                ..default()
            })
            .add_systems(Startup, Self::startup_driver)
            .add_systems(
                Update,
                Self::write_to_console_driver.after(ConsoleSet::ConsoleUI),
            )
            .add_systems(Update, Self::resize_driver.before(ConsoleSet::ConsoleUI))
            .add_systems(
                Update,
                Self::raw_commands_driver.in_set(ConsoleSet::Commands),
            )
            .add_console_command::<LogCommand, _>(LogCommand::driver);
    }

    fn raw_commands_driver(mut console_commands: EventReader<ConsoleCommandEntered>) {
        for ConsoleCommandEntered { command_name, args } in console_commands.read() {
            println!(r#"Entered command "{command_name}" with args {:#?}"#, args);
        }
    }

    // a dummy to remember that anything printing to console should go after consoleUI
    pub fn write_to_console_driver() {
        // console_line.write(PrintConsoleLine::new("Hello".into()));
    }

    pub fn resize_driver(
        mut config: ResMut<ConsoleConfiguration>,
        query: Query<&Window>,
    ) -> Result {
        let window = query.single()?;
        config.width = window.width() - PADDING * 2.0;
        config.height = window.height() * 2.0 - PADDING * 2.0;
        Ok(())
    }

    pub fn startup_driver(mut console: Console) {
        // reply!(log, "my bevy is ready");
        console.log("My Bevy Is Ready");
    }
}

/// Prints given arguments to the console
#[derive(Parser, ConsoleCommand)]
#[command(name = "log")]
struct LogCommand {
    /// Message to print
    msg: String,
    /// Number of times to print message
    num: Option<i64>,
}
impl LogCommand {
    fn driver(mut log: ConsoleCommand<LogCommand>) {
        if let Some(Ok(LogCommand { msg, num })) = log.take() {
            let repeat_count = num.unwrap_or(1);

            for _ in 0..repeat_count {
                reply!(log, "{msg}");
            }

            // log.ok();
        }
    }
}
