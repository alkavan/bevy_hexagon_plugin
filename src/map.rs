use bevy::prelude::*;
use std::collections::HashMap;
use hexagon_tiles::hexagon::Hex;
use hexagon_tiles::layout::Layout;

type HexTilemap = HashMap<Hex, String>;

#[derive(Resource)]
pub struct HexMap {
    pub(crate) map: HexTilemap,
    pub(crate) layout: Layout,
}

impl HexMap {
    pub fn new(map: HexTilemap, layout: Layout) -> Self {
        Self { map, layout }
    }
}
