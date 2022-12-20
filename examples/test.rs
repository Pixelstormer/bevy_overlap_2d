use bevy::prelude::*;
use bevy_overlap_2d::{
    Circle, Collider, ColliderBundle, ColliderDrawBundle, Collides, CollisionPlugin,
};

#[derive(Resource, Default)]
struct CursorPosition(Vec2);

#[derive(Resource, Default)]
struct CurrentPicked {
    pub entity: Option<Entity>,
    pub offset: Vec2,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CollisionPlugin)
        .init_resource::<CurrentPicked>()
        .init_resource::<CursorPosition>()
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
        ColliderBundle {
            collider: Circle::new(Vec2::ZERO, 25.0).into(),
            ..Default::default()
        },
        ColliderDrawBundle::default(),
    ));
}

fn update_cursor_position(
    query: Query<(&Camera, &GlobalTransform)>,
    windows: Res<Windows>,
    mut cursor_position: ResMut<CursorPosition>,
) {
    let (camera, camera_transform) = query.single();
    let Some(viewport_position) = windows.primary().cursor_position() else { return };
    let Some(world_position) = camera.viewport_to_world(camera_transform, viewport_position) else { return };
    *cursor_position = CursorPosition(world_position.origin.truncate());
}

fn pick(
    query: Query<(Entity, &Transform, &Collider)>,
    mouse: Res<Input<MouseButton>>,
    cursor_position: Res<CursorPosition>,
    mut picked: ResMut<CurrentPicked>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        for (entity, transform, collider) in query.iter() {
            let result = collider.collide(&cursor_position.0.into());
            if result.colliding {
                picked.entity = Some(entity);
                picked.offset = cursor_position.0 - transform.translation.truncate();
                break;
            }
        }
    }
}

fn move_picked(
    mut query: Query<&mut Transform>,
    mouse: Res<Input<MouseButton>>,
    cursor: Res<CursorPosition>,
    picked: Res<CurrentPicked>,
) {
    if mouse.pressed(MouseButton::Left) {
        if let Some(entity) = picked.entity {
            let mut transform = query.get_mut(entity).unwrap();
            transform.translation = (cursor.0 + picked.offset).extend(0.0);
        }
    }
}

fn unpick(mouse: Res<Input<MouseButton>>, mut picked: ResMut<CurrentPicked>) {
    if mouse.just_released(MouseButton::Left) {
        picked.entity = None;
    }
}
