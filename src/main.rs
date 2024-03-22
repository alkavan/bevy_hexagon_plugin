mod map;
mod plugin;
mod shapes;

use crate::map::HexMap;
use crate::plugin::create_flat_layout;
use crate::shapes::Hexagons;
use bevy::prelude::*;
use bevy::utils::HashMap;
use plugin::HexMapPlugin;

fn main() {
    let tile_size = Vec2::new(20.0, 20.0);
    let layout = create_flat_layout(tile_size, tile_size / 2.0);
    let mut map = HexMap::new(HashMap::new(), layout);
    map.build(5);

    App::new()
        .insert_resource(map)
        .add_plugins((DefaultPlugins, HexMapPlugin))
        .add_systems(PostStartup, camera_system)
        .run();
}

fn camera_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
