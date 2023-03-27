use crate::map::HexMap;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
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
use std::ops::Add;
use std::slice::Windows;

pub struct HexMapPlugin;

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
    let window = windows.single();
    let ww = window.resolution.width();
    let wh = window.resolution.height();

    for (key, value) in &map_resource.map {
        let hexagon = bevy_prototype_lyon::prelude::RegularPolygon {
            sides: 6,
            center: Vec2::ZERO,
            feature: RegularPolygonFeature::Radius(20.0),
            ..shapes::RegularPolygon::default()
        };

        let point = LayoutTool::hex_to_pixel(map_resource.layout, *key);

        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&hexagon),
                transform: Transform::from_xyz(point.x as f32, point.y as f32, 0.0),
                ..default()
            },
            Fill::color(Color::TEAL),
            Stroke::new(Color::RED, 1.0),
            HexagonComponent(*key),
        ));

        // spawn hexagon position texts
        commands.spawn((
            // Create a TextBundle that has a Text with a list of sections.
            TextBundle::from_sections([TextSection::new(
                format!("({}, {}, {})", key.q(), key.r(), key.s()),
                TextStyle {
                    font: asset_server.load("fonts/RobotoMono-Regular.ttf"),
                    font_size: 11.0,
                    color: Color::WHITE,
                },
            )])
            .with_text_alignment(TextAlignment::Center)
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px((point.y as f32).add(wh / 2.)),
                    left: Val::Px((point.x as f32).add(ww / 2.)),
                    ..default()
                },
                ..default()
            }),
            HexagonText,
        ));
    }
}

fn update_map(
    mut transforms: Query<(&mut Transform, &HexagonComponent)>,
    map_resource: Res<HexMap>,
) {
    // for (mut transform, component) in transforms.iter_mut() {
    //     let hex: Hex = component.0;
    //     let point = LayoutTool::hex_to_pixel(map_resource.layout, hex);
    //     transform.translation = vec3(point.x as f32, point.y as f32, 0.0);
    // }
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
