use bevy::prelude::*;
use bevy_overlap_2d::{Circle, ColliderBundle, ColliderDrawBundle, Colliding, CollisionPlugin};

#[derive(Component)]
struct Cursor;

#[derive(Component, Default)]
struct Picked {
    pub entity: Option<Entity>,
    pub offset: Vec3,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CollisionPlugin)
        .add_startup_system(spawn_world)
        .add_system(update_cursor_position)
        .add_system(pick.after(update_cursor_position))
        .add_system(move_picked.after(pick))
        .add_system(unpick.after(move_picked))
        .run()
}

fn spawn_world(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        ColliderBundle::point(Vec2::ZERO),
        Cursor,
        Picked::default(),
        ColliderDrawBundle::default(),
    ));

    commands
        .spawn((
            ColliderBundle::capsule(30.0, 15.0),
            ColliderDrawBundle::default(),
        ))
        .insert(Transform::from_xyz(-150.0, 0.0, 0.0));

    commands
        .spawn((ColliderBundle::circle(25.0), ColliderDrawBundle::default()))
        .insert(Transform::from_xyz(-100.0, 0.0, 0.0));

    commands
        .spawn((
            ColliderBundle::line(Vec2::new(-10.0, -25.0), Vec2::new(10.0, 25.0)),
            ColliderDrawBundle::default(),
        ))
        .insert(Transform::from_xyz(-50.0, 0.0, 0.0));

    commands
        .spawn((
            ColliderBundle::point(Vec2::ZERO),
            ColliderDrawBundle::default(),
        ))
        .insert(Transform::from_xyz(50.0, 0.0, 0.0));

    commands
        .spawn((
            ColliderBundle::polygon([
                Vec2::new(-15.0, -25.0),
                Vec2::new(15.0, -25.0),
                Vec2::new(25.0, 25.0),
                Vec2::new(-25.0, 25.0),
            ]),
            ColliderDrawBundle::default(),
        ))
        .insert(Transform::from_xyz(100.0, 0.0, 0.0));

    commands
        .spawn((
            ColliderBundle::rect(Rect::from_center_half_size(
                Vec2::ZERO,
                Vec2::new(15.0, 30.0),
            )),
            ColliderDrawBundle::default(),
        ))
        .insert(Transform::from_xyz(150.0, 0.0, 0.0));

    // commands
    //     .spawn((
    //         ColliderBundle::triangle(
    //             Vec2::new(0.0, 25.0),
    //             Vec2::new(15.0, 15.0),
    //             Vec2::new(-15.0, 15.0),
    //         ),
    //         ColliderDrawBundle::default(),
    //     ))
    //     .insert(Transform::from_xyz(150.0, 0.0, 0.0));
}

fn update_cursor_position(
    mut cursor_query: Query<&mut Transform, With<Cursor>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Res<Windows>,
) {
    let (camera, camera_transform) = camera_query.single();
    let Some(viewport_position) = windows.primary().cursor_position() else { return };
    let Some(world_position) = camera.viewport_to_world(camera_transform, viewport_position) else { return };
    for mut transform in cursor_query.iter_mut() {
        transform.translation = world_position.origin;
    }
}

fn pick(
    mut cursor_query: Query<(&Transform, &Colliding, &mut Picked), With<Cursor>>,
    picked_query: Query<&Transform, Without<Cursor>>,
    mouse: Res<Input<MouseButton>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        for (transform, colliding, mut picked) in cursor_query.iter_mut() {
            if let Some(&entity) = colliding.0.iter().next() {
                let picked_transform = picked_query.get(entity).unwrap();
                picked.entity = Some(entity);
                picked.offset = picked_transform.translation - transform.translation;
            }
        }
    }
}

fn move_picked(
    cursor_query: Query<(&Transform, &Picked), With<Cursor>>,
    mut picked_query: Query<&mut Transform, Without<Cursor>>,
    mouse: Res<Input<MouseButton>>,
) {
    if mouse.pressed(MouseButton::Left) {
        for (transform, picked) in cursor_query.iter() {
            if let Some(mut picked_transform) = picked
                .entity
                .and_then(|entity| picked_query.get_mut(entity).ok())
            {
                picked_transform.translation = transform.translation + picked.offset;
            }
        }
    }
}

fn unpick(mut query: Query<&mut Picked, With<Cursor>>, mouse: Res<Input<MouseButton>>) {
    if mouse.just_released(MouseButton::Left) {
        for mut picked in query.iter_mut() {
            picked.entity = None;
        }
    }
}
