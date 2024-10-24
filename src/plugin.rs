use crate::map::HexMap;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use hexagon_tiles::hexagon::Hex;
use hexagon_tiles::layout::{
    Layout,
    LayoutTool,
    LAYOUT_ORIENTATION_FLAT,
    //LAYOUT_ORIENTATION_POINTY,
};
use hexagon_tiles::point::Point;

pub struct HexMapPlugin;

impl Plugin for HexMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ShapePlugin)
            .add_systems(Startup, setup_map)
            .add_systems(Update, update_map);
    }
}

#[derive(Component)]
struct HexComponent(Hex);

fn setup_map(mut commands: Commands, map_resource: Res<HexMap>) {
    for (key, value) in &map_resource.map {
        let hexagon = bevy_prototype_lyon::prelude::RegularPolygon {
            sides: 6,
            center: Vec2::ZERO,
            feature: RegularPolygonFeature::Radius(20.0),
            ..shapes::RegularPolygon::default()
        };

        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&hexagon),
                ..default()
            },
            Fill::color(Color::TEAL),
            Stroke::new(Color::RED, 1.0),
            HexComponent(*key),
        ));
    }
}

fn update_map(mut transforms: Query<(&mut Transform, &HexComponent)>, map_resource: Res<HexMap>) {
    for (mut transform, component) in transforms.iter_mut() {
        let hex: Hex = component.0;
        let point = LayoutTool::hex_to_pixel(map_resource.layout, hex);
        transform.translation = vec3(point.x as f32, point.y as f32, 0.0);
    }
}

pub fn create_flat_layout(size: Vec2, origin: Vec2) -> Layout {
    return Layout {
        orientation: LAYOUT_ORIENTATION_FLAT,
        size: Point {
            x: size.x as f64,
            y: size.y as f64,
        },
        origin: Point {
            x: origin.x as f64,
            y: origin.y as f64,
        },
    };
}
