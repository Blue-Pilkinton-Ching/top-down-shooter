use bevy::prelude::*;

use crate::plugins::*;

mod plugins;
fn main() {
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: "Game".to_string(),
            canvas: Some("#game_canvas".to_string()),
            ..Default::default()
        }),
        ..Default::default()
    };
    let default_plugins = DefaultPlugins.set(window_plugin);
    App::new()
        .add_plugins(default_plugins)
        .add_plugins((
            input::InputPlugin,
            util::UtilPlugins,
            player::PlayerPlugin,
            world::WorldPlugin,
        ))
        .run();
}
