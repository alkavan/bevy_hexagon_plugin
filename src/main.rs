mod map;
mod plugin;
mod shapes;
mod debug;

use bevy::prelude::*;

use crate::map::HexMap;
use crate::plugin::create_flat_layout;
use crate::shapes::Hexagons;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

use bevy::utils::HashMap;
use bevy::window::{PresentMode, WindowResolution};
use plugin::HexMapPlugin;
use crate::debug::DebugGridPlugin;

fn main() {
    let window_resolution = WindowResolution::new(1280., 720.);
    let canvas_size = Vec2::new(window_resolution.width(), window_resolution.height());

    // hex tilemap
    let tile_size = Vec2::new(32., 32.);
    let layout = create_flat_layout(tile_size, tile_size / 2.0);
    let mut hex_map = HexMap::new(HashMap::new(), layout);
    hex_map.build(5);

    // debug grid
    let grid_size = Vec2::new(16., 16.);

    let preset_mode = PresentMode::AutoVsync;
    App::new()
        .insert_resource(hex_map)
        .add_systems(PostStartup, camera_system)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Hexagon Plugin".into(),
                resolution: window_resolution,
                present_mode: preset_mode,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            HexMapPlugin{ tile_size },
            DebugGridPlugin{ grid_size, canvas_size },
            FrameTimeDiagnosticsPlugin::default()
        ))
        .run();
}

fn camera_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
