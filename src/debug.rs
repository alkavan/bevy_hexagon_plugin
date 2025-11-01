
use bevy::{prelude::*, color::palettes::css::*};
use bevy_prototype_lyon::prelude::*;

use bevy::math::{Vec2};
use std::collections::HashMap;

pub struct DebugGridPlugin {
    pub grid_size: Vec2,
    pub canvas_size: Vec2,
}

#[derive(Copy, Clone)]
enum GridLineOrientation {
    Vertical,
    Horizontal,
}

type GridLineArray = HashMap<u16, f32>;

#[derive(Resource)]
pub struct GridLayout {
    pub(crate) map: GridLineArray,
}

impl GridLayout {
    pub fn new(map: GridLineArray) -> Self {
        Self { map }
    }
}


#[derive(Component)]
struct GridComponent(GridLineOrientation);

impl Plugin for DebugGridPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GridLayout::new(HashMap::new()));
        app
            .add_systems(Startup, setup_grid)
            .add_systems(Update, update_grid);
    }
}

fn setup_grid(mut commands: Commands) {
    // Create a rectangle using bevy_prototype_lyon's Polygon shape
    // Define the four corners of a rectangle
    let points = vec![
        Vec2::new(0.0, 0.0),
        Vec2::new(100.0, 0.0),
        Vec2::new(100.0, 100.0),
        Vec2::new(0.0, 100.0),
    ];

    let polygon = shapes::Polygon {
        points,
        closed: true,
    };

    // Spawn with ShapeBuilder (the correct API for bevy_prototype_lyon 0.15.0)
    commands.spawn(
        ShapeBuilder::with(&polygon)
            .fill(Color::srgb(0.0, 1.0, 0.0))
            .stroke((Color::srgb(1.0, 0.0, 0.0), 2.0))
            .build()
    );
}

fn update_grid(
    mut transforms: Query<(&mut Transform, &GridComponent)>,
    map_resource: Res<GridLayout>,
) {
    for (mut transform, component) in transforms.iter_mut() {
        let orientation: GridLineOrientation = component.0;
        // let point = LayoutTool::hex_to_pixel(map_resource.layout, hex);
        // transform.translation = vec3(point.x as f32, point.y as f32, 0.0);
    }
}