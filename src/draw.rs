use crate::Collider;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{DrawMode, GeometryBuilder, StrokeMode};

#[derive(Clone, Copy, Component, Default)]
pub struct DrawCollider;

#[derive(Clone, Copy, Component)]
pub struct DrawColliderShape(Entity);

pub fn draw_colliders(
    mut commands: Commands,
    query: Query<(Entity, &Collider), Added<DrawCollider>>,
) {
    for (entity, collider) in query.iter() {
        let mut e = commands.spawn(GeometryBuilder::build_as(
            collider,
            DrawMode::Stroke(StrokeMode::color(Color::GREEN)),
            Transform::default(),
        ));
        e.set_parent(entity);

        let id = e.id();
        commands.entity(entity).insert(DrawColliderShape(id));
    }
}

pub fn update_colliders(
    mut commands: Commands,
    query: Query<(&Collider, &DrawColliderShape), Changed<Collider>>,
) {
    for (collider, DrawColliderShape(entity)) in query.iter() {
        commands.entity(*entity).insert(GeometryBuilder::build_as(
            collider,
            DrawMode::Stroke(StrokeMode::color(Color::GREEN)),
            Transform::default(),
        ));
    }
}

pub fn undraw_colliders(
    mut commands: Commands,
    removed_collider: RemovedComponents<Collider>,
    removed_draw: RemovedComponents<DrawCollider>,
    query: Query<Option<&DrawColliderShape>>,
) {
    for entity in removed_collider.iter() {
        if let Ok(Some(DrawColliderShape(e))) = query.get(entity) {
            commands.entity(*e).despawn();
            commands.entity(entity).remove::<DrawColliderShape>();
        }
    }

    for entity in removed_draw.iter() {
        if let Ok(Some(DrawColliderShape(e))) = query.get(entity) {
            commands.entity(*e).despawn();
            commands.entity(entity).remove::<DrawColliderShape>();
        }
    }
}
