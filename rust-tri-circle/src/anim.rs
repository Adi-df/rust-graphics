use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

// Figure components
#[derive(Component)]
pub struct Circle;

#[derive(Component)]
pub struct RegularPolygon {
    pub sides: u16,
}

#[derive(Component)]
pub struct ShapeRadius {
    pub radius: f32,
}

#[derive(Component)]
pub struct ArroundCenter {
    pub radius: f32,
    pub angle: f32,
}

// Animation components
#[derive(Component)]
pub struct Rotating {
    pub speed: f32,
}

#[derive(Component)]
pub struct Moving {
    pub direction: Vec3,
}

#[derive(Component)]
pub struct Growing {
    pub factor: Vec3,
}

#[derive(Component)]
pub struct TurningArround {
    pub angle: f32,
}

// Animate to components
#[derive(Component)]
pub struct RotatingTo {
    pub rotation: f32,
    pub steps: f32,
}

#[derive(Component)]
pub struct MovingTo {
    pub position: Vec3,
    pub steps: f32,
}

#[derive(Component)]
pub struct GrowingTo {
    pub factor: Vec3,
    pub steps: f32,
}

#[derive(Component)]
pub struct TurningArroundTo {
    pub angle: f32,
    pub steps: f32,
}

// Animation chaining
#[derive(Component)]
pub struct AnimationIndex {
    pub index: usize,
    pub max: Option<usize>,
    pub looping: bool,
}

// Plugin
pub struct BasicAnim;

impl Plugin for BasicAnim {
    fn build(&self, app: &mut App) {
        app
            // Shapes updating
            .add_system(update_circles_radius)
            .add_system(update_regular_polygons_radius_or_sides)
            // Position move
            .add_system(update_position_on_arround_center_change)
            // Infinite animations
            .add_system(rotate_shapes)
            .add_system(grow_shapes)
            .add_system(move_shapes)
            .add_system(turn_arround_shapes)
            // To animations
            .add_system(rotate_to_shapes)
            .add_system(move_to_shapes)
            .add_system(grow_to_shapes)
            .add_system(turn_arround_to_shapes);
    }
}

// Update circles
fn update_circles_radius(
    mut circles: Query<(&mut Path, &ShapeRadius), (Changed<ShapeRadius>, With<Circle>)>,
) {
    circles
        .iter_mut()
        .for_each(|(mut path, ShapeRadius { radius })| {
            *path = ShapePath::new()
                .add(&shapes::Circle {
                    center: Vec2::ZERO,
                    radius: *radius,
                })
                .build();
        });
}
fn update_regular_polygons_radius_or_sides(
    mut polygons: Query<
        (&mut Path, &ShapeRadius, &RegularPolygon),
        Or<(Changed<ShapeRadius>, Changed<RegularPolygon>)>,
    >,
) {
    polygons.iter_mut().for_each(
        |(mut path, ShapeRadius { radius }, RegularPolygon { sides })| {
            *path = ShapePath::new()
                .add(&shapes::RegularPolygon {
                    center: Vec2::ZERO,
                    sides: *sides as usize,
                    feature: RegularPolygonFeature::Radius(*radius),
                })
                .build();
        },
    );
}
fn update_position_on_arround_center_change(
    mut shapes: Query<(&mut Transform, &ArroundCenter), Changed<ArroundCenter>>,
) {
    shapes
        .iter_mut()
        .for_each(|(mut transform, ArroundCenter { radius, angle })| {
            transform.translation.x = radius * angle.cos();
            transform.translation.y = radius * angle.sin();
        });
}

fn rotate_shapes(mut shapes: Query<(&mut Transform, &Rotating)>) {
    shapes
        .iter_mut()
        .for_each(|(mut transform, Rotating { speed })| {
            transform.rotate(Quat::from_rotation_z(*speed));
        });
}
fn grow_shapes(mut shapes: Query<(&mut Transform, &Growing)>) {
    shapes
        .iter_mut()
        .for_each(|(mut transform, Growing { factor })| {
            transform.scale += *factor;
        });
}
fn move_shapes(mut shapes: Query<(&mut Transform, &Moving)>) {
    shapes
        .iter_mut()
        .for_each(|(mut transform, Moving { direction })| transform.translation += *direction)
}
fn turn_arround_shapes(mut shapes: Query<(&mut ArroundCenter, &TurningArround)>) {
    shapes
        .iter_mut()
        .for_each(|(mut arround_center, TurningArround { angle })| {
            arround_center.angle += *angle;
        });
}

fn update_animation_index(animation_index: Option<Mut<'_, AnimationIndex>>) {
    if let Some(mut animation_index) = animation_index {
        animation_index.index += 1;

        if let Some(max) = animation_index.max {
            if animation_index.index == max && animation_index.looping {
                animation_index.index = 0;
            }
        }
    }
}

fn rotate_to_shapes(
    mut commands: Commands,
    mut shapes: Query<(
        Entity,
        &mut Transform,
        &mut RotatingTo,
        Option<&mut AnimationIndex>,
    )>,
) {
    shapes
        .iter_mut()
        .for_each(|(entity, mut transform, mut rotating, animation_index)| {
            if rotating.steps > 0. {
                transform.rotation = transform.rotation.lerp(
                    Quat::from_rotation_z(rotating.rotation),
                    1.0 / rotating.steps,
                );
                rotating.steps -= 1.;
            } else {
                commands.entity(entity).remove::<RotatingTo>();

                update_animation_index(animation_index);
            }
        })
}
fn move_to_shapes(
    mut commands: Commands,
    mut shapes: Query<(
        Entity,
        &mut Transform,
        &mut MovingTo,
        Option<&mut AnimationIndex>,
    )>,
) {
    shapes
        .iter_mut()
        .for_each(|(entity, mut transform, mut moving, animation_index)| {
            if moving.steps > 0. {
                transform.translation = transform
                    .translation
                    .lerp(moving.position, 1.0 / moving.steps);
                moving.steps -= 1.;
            } else {
                commands.entity(entity).remove::<MovingTo>();

                update_animation_index(animation_index);
            }
        })
}
fn grow_to_shapes(
    mut commands: Commands,
    mut shapes: Query<(
        Entity,
        &mut Transform,
        &mut GrowingTo,
        Option<&mut AnimationIndex>,
    )>,
) {
    shapes
        .iter_mut()
        .for_each(|(entity, mut transform, mut growing, animation_index)| {
            if growing.steps > 0. {
                transform.scale = transform.scale.lerp(growing.factor, 1.0 / growing.steps);
                growing.steps -= 1.;
            } else {
                commands.entity(entity).remove::<GrowingTo>();

                update_animation_index(animation_index);
            }
        })
}
fn turn_arround_to_shapes(
    mut commands: Commands,
    mut shapes: Query<(
        Entity,
        &mut ArroundCenter,
        &mut TurningArroundTo,
        Option<&mut AnimationIndex>,
    )>,
) {
    shapes.iter_mut().for_each(
        |(entity, mut arround_center, mut turning_arround, animation_index)| {
            if turning_arround.steps > 0. {
                arround_center.angle +=
                    (turning_arround.angle - arround_center.angle) / turning_arround.steps;
                turning_arround.steps -= 1.;
            } else {
                commands.entity(entity).remove::<TurningArroundTo>();

                update_animation_index(animation_index);
            }
        },
    );
}
