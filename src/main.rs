mod plugin;

use crate::plugin::{create_flat_layout, HexMap, Hexagons};
use bevy::prelude::*;
use plugin::HexMapPlugin;
use std::collections::HashMap;

fn main() {
    let tile_size = Vec2::new(20.0, 20.0);
    let layout = create_flat_layout(tile_size, tile_size / 2.0);
    let mut map = HexMap::new(HashMap::new(), layout);
    map.build(5);

    App::new()
        .insert_resource(map)
        .add_plugins(DefaultPlugins)
        .add_plugin(HexMapPlugin)
        .add_startup_system(camera_system)
        .run();
}

fn camera_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
