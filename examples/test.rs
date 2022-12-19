use bevy::prelude::*;
use bevy_overlap_2d::{Circle, ColliderBundle, ColliderDrawBundle, CollisionPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CollisionPlugin)
        .add_startup_system(spawn_world)
        .run()
}

fn spawn_world(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        ColliderBundle {
            collider: Circle::new(Vec2::ZERO, 5.0).into(),
            ..Default::default()
        },
        ColliderDrawBundle::default(),
    ));
}
