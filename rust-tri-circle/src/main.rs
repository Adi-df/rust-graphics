use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

mod anim;

const TAU: f32 = PI * 2.;

const SIDES: u8 = 3;
const LINES: u8 = 20;

const SPEED: f32 = PI / 120.;
const SUB_SPEEDS: f32 = SPEED * 2.;

const MAIN_RADIUS: f32 = 180.;
const SUB_RADIUS: f32 = 80.;

#[derive(Component)]
pub struct LinePoint(usize);

#[derive(Component)]
pub struct Line(usize);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(anim::BasicAnim)
        .insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(setup)
        .add_system(update_line)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands
        .spawn()
        .insert(anim::Circle)
        .insert(anim::ShapeRadius {
            radius: MAIN_RADIUS,
        })
        .insert_bundle(GeometryBuilder::build_as(
            &ShapePath::new().build(),
            DrawMode::Stroke(StrokeMode::new(Color::WHITE, 4.)),
            Transform::from_translation(Vec2::ZERO.extend(0.)),
        ))
        .with_children(|parent| {
            (0..SIDES).for_each(|side| {
                parent
                    .spawn()
                    .insert(anim::Circle)
                    .insert(anim::ShapeRadius { radius: SUB_RADIUS })
                    .insert(anim::ArroundCenter {
                        angle: TAU / SIDES as f32 * side as f32,
                        radius: MAIN_RADIUS,
                    })
                    .insert(anim::TurningArround { angle: SPEED })
                    .insert_bundle(GeometryBuilder::build_as(
                        &ShapePath::new().build(),
                        DrawMode::Stroke(StrokeMode::new(Color::WHITE, 5.)),
                        Transform::from_translation(Vec2::ZERO.extend(0.)),
                    ))
                    .with_children(|parent| {
                        (0..LINES).for_each(|line| {
                            parent
                                .spawn()
                                .insert(LinePoint(line as usize))
                                .insert_bundle(TransformBundle::from_transform(
                                    Transform::from_translation(Vec2::ZERO.extend(0.)),
                                ))
                                .insert(anim::ArroundCenter {
                                    // angle: TAU / LINES as f32 * line as f32,
                                    angle: TAU / SIDES as f32 * side as f32
                                        + TAU / LINES as f32 * line as f32,
                                    radius: SUB_RADIUS,
                                })
                                .insert(anim::TurningArround { angle: SUB_SPEEDS });
                        });
                    });
            });
        });

    (0..LINES).for_each(|line| {
        commands
            .spawn()
            .insert(Line(line as usize))
            .insert_bundle(GeometryBuilder::build_as(
                &ShapePath::new().build(),
                DrawMode::Stroke(StrokeMode::new(Color::WHITE, 3.)),
                Transform::from_translation(Vec2::ZERO.extend(0.)),
            ));
    });
}

fn update_line(
    lines_points: Query<(&GlobalTransform, &LinePoint), Changed<GlobalTransform>>,
    mut lines: Query<(&mut Path, &Line)>,
) {
    lines.iter_mut().for_each(|(mut path, Line(id))| {
        let mut new_path = PathBuilder::new();

        lines_points
            .iter()
            .filter(|(_, LinePoint(line_id))| *id == *line_id)
            .for_each(|(transform, _)| {
                new_path.line_to(transform.translation.truncate());
            });
        new_path.close();

        *path = new_path.build();
    });
}
