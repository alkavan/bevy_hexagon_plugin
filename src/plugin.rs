use crate::map::HexMap;

use bevy::math::vec3;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::{color::palettes::css::*};
use bevy_prototype_lyon::prelude::*;
use hexagon_tiles::hexagon::Hex;
use hexagon_tiles::layout::{
    Layout,
    LayoutTool,
    LAYOUT_ORIENTATION_FLAT,
    //LAYOUT_ORIENTATION_POINTY,
};
use hexagon_tiles::point::Point;

pub struct HexMapPlugin {
    pub tile_size: Vec2
}

impl Plugin for HexMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ShapePlugin)
            .add_systems(Startup, setup_map)
            .add_systems(Update, update_map);
    }
}

#[derive(Component)]
struct HexagonComponent(Hex);

#[derive(Component)]
struct HexagonText;

fn setup_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map_resource: Res<HexMap>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window: &Window = windows.single().unwrap();

    let ww: f32 = window.width();
    let wh: f32 = window.height();

    for (key, value) in &map_resource.map {
        let hexagon = shapes::RegularPolygon {
            sides: 6,
            feature: shapes::RegularPolygonFeature::Radius(32.0),
            ..default()
        };

        let point: Point = LayoutTool::hex_to_pixel(map_resource.layout, *key);

        commands.spawn((
            ShapeBuilder::with(&hexagon)
                .fill(TEAL)
                .stroke((RED, 1.0))
                .build(),
            HexagonComponent(*key),
        ));

        // spawn hexagon position texts
        commands.spawn((
            Text::new(format!("{},{},{}", key.q(), key.r(), key.s())),
            TextFont {
                font: asset_server.load("fonts/RobotoMono-Regular.ttf").into(),
                font_size: 11.0,
                ..default()
            },
            TextColor(Color::WHITE),
            TextLayout {
                justify: Justify::Center,
                ..default()
            },
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(point.y as f32 + wh / 2.0),
                left: Val::Px(point.x as f32 + ww / 2.0),
                ..default()
            },
            HexagonText,
        ));
    }
}

fn update_map(
    mut transforms: Query<(&mut Transform, &HexagonComponent)>,
    map_resource: Res<HexMap>,
) {
    for (mut transform, component) in transforms.iter_mut() {
        let hex: Hex = component.0;
        let point = LayoutTool::hex_to_pixel(map_resource.layout, hex);
        transform.translation = vec3(point.x as f32, point.y as f32, 0.0);
    }
}

fn text_update_system(
    mut transforms: Query<&mut Transform, With<HexagonText>>,
    map_resource: Res<HexMap>,
) {
    // for (mut transform, component) in transforms.iter_mut() {}
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
