mod map;
mod plugin;
mod shapes;
mod debug;

use crate::map::HexMap;
use crate::plugin::create_flat_layout;
use crate::shapes::Hexagons;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::window::PresentMode;
use plugin::HexMapPlugin;

fn main() {
    let tile_size = Vec2::new(32.0, 32.0);
    let layout = create_flat_layout(tile_size, tile_size / 2.0);
    let mut map = HexMap::new(HashMap::new(), layout);
    map.build(5);

    App::new()
        .insert_resource(map)
        .add_systems(PostStartup, camera_system)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Hexagon Plugin".into(),
                resolution: (1280., 720.).into(),
                present_mode: PresentMode::AutoVsync,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            HexMapPlugin{tile_size},
            FrameTimeDiagnosticsPlugin::default()
        ))
        .run();
}

fn camera_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
