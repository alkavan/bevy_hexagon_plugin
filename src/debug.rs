use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use bevy::math::{Vec2};
use bevy::utils::HashMap;

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
        app
            .add_systems(Startup, setup_grid)
            .add_systems(Update, update_grid);
    }
}

fn setup_grid(mut commands: Commands) {
    let line = shapes::Line(Vec2::new(0.0, 0.0), Vec2::new(10.0, 0.0));

    let square = shapes::Rectangle {
        extents: Vec2::splat(100.0),
        ..shapes::Rectangle::default()
    };

    let mut builder = GeometryBuilder::new().add(&line).add(&square);

    commands.spawn((
        ShapeBundle {
            path: builder.build(),
            ..default()
        },
        Fill::color(Color::BLUE),
        Stroke::new(Color::BLUE, 1.0),
    ));
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