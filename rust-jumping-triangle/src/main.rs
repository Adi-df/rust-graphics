use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

const TAU: f32 = PI * 2.;
const TRIANGLE_SIZE: f32 = 60.;

#[derive(Component)]
pub struct Speed(Vec2);

#[derive(Component)]
pub struct Gravity(f32);

#[derive(Component)]
pub struct Bounce {
    factor: f32,
}

pub struct SpawnTimer(Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(SpawnTimer(Timer::new(Duration::from_millis(500), true)))
        .add_startup_system(setup)
        .add_system(integrate_speed)
        .add_system(apply_gravity)
        .add_system(bounce_on_sides.before(integrate_speed))
        .add_system(loop_spawn)
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d());

    commands
        .spawn()
        .insert(Speed(Vec2::new(1.1, 0.)))
        .insert(Gravity(0.1))
        .insert(Bounce { factor: 1. })
        .insert_bundle(GeometryBuilder::build_as(
            &ShapePath::build_as(&RegularPolygon {
                sides: 3,
                center: Vec2::ZERO,
                feature: RegularPolygonFeature::Radius(TRIANGLE_SIZE),
            }),
            DrawMode::Stroke(StrokeMode::new(Color::WHITE, 4.)),
            Transform::default(),
        ));
}

fn integrate_speed(mut shapes: Query<(&mut Transform, &Speed)>) {
    shapes
        .iter_mut()
        .for_each(|(mut transform, Speed(speed))| transform.translation += speed.extend(0.));
}

fn apply_gravity(mut shapes: Query<(&mut Speed, &Gravity)>) {
    shapes
        .iter_mut()
        .for_each(|(mut speed, Gravity(force))| speed.0 += Vec2::Y * -force);
}

fn bounce_on_sides(
    windows: Res<Windows>,
    mut shapes: Query<(&mut Transform, &mut Speed, &Bounce, &GlobalTransform)>,
) {
    let window = windows.primary();

    shapes.iter_mut().for_each(
        |(mut transform, mut speed, Bounce { factor }, GlobalTransform { translation, .. })| {
            let half_height = TRIANGLE_SIZE * (TAU * (11. / 12.)).sin();
            let right_down = translation.truncate()
                + Vec2::new(TRIANGLE_SIZE * (TAU * (11. / 12.)).cos(), half_height);

            let left_down = translation.truncate()
                + Vec2::new(TRIANGLE_SIZE * (TAU * (7. / 12.)).cos(), half_height);

            if right_down.y.abs() >= window.height() / 2. {
                speed.0.y *= -factor;
            }

            if right_down.x.abs() >= window.width() / 2. || left_down.x.abs() >= window.width() / 2.
            {
                speed.0.x *= -factor;
            }
        },
    );
}

fn loop_spawn(mut commands: Commands, time: Res<Time>, mut timer: ResMut<SpawnTimer>) {
    if timer.0.tick(time.delta()).just_finished() {
        commands
            .spawn()
            .insert(Speed(Vec2::new(1.1, 0.)))
            .insert(Gravity(0.1))
            .insert(Bounce { factor: 1. })
            .insert_bundle(GeometryBuilder::build_as(
                &ShapePath::build_as(&RegularPolygon {
                    sides: 3,
                    center: Vec2::ZERO,
                    feature: RegularPolygonFeature::Radius(TRIANGLE_SIZE),
                }),
                DrawMode::Stroke(StrokeMode::new(Color::WHITE, 4.)),
                Transform::default(),
            ));
    }
}
