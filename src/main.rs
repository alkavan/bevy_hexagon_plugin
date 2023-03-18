mod plugin;

use bevy::prelude::*;

use plugin::HexMapPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HexMapPlugin)
        .add_startup_system(camera_system)
        .run();
}

fn camera_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
