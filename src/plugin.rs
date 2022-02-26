use bevy::math::vec3;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use hexagon_tiles::hexagon::Hex;

pub struct HexMapPlugin;

impl Plugin for HexMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ShapePlugin)
            .add_startup_system(setup_map.system())
            .add_system(update_map.system());
    }
}

#[derive(Component)]
struct HexComponent(Hex);

fn setup_map(mut commands: Commands) {
    for q in 1..3 {
        for r in 1..3 {
            let hexagon = shapes::RegularPolygon {
                sides: 6,
                center: Vec2::ZERO,
                feature: shapes::RegularPolygonFeature::Radius(20.0),
                ..shapes::RegularPolygon::default()
            };

            let shape_bundle = GeometryBuilder::build_as(
                &hexagon,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::TEAL),
                    outline_mode: StrokeMode::new(Color::RED, 1.0),
                },
                Transform::default(),
            );

            commands
                .spawn_bundle(shape_bundle)
                .insert(HexComponent(Hex::new(q, r)));
        }
    }
}

fn update_map(mut transforms: Query<(&mut Transform, &HexComponent)>) {
    for (mut transform, component) in transforms.iter_mut() {
        // transform.translation = transform.translation + vec3(1.0, 1.0, 1.0);
        // let x = transform.translation.x;
        // let y = transform.translation.y;
        let hex: Hex = component.0;
        println!("Hex ({}, {})", hex.q(), hex.r())
    }
}
