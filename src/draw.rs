use crate::{Collider, Colliding};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{DrawMode, GeometryBuilder, StrokeMode};

#[derive(Clone, Copy, Component, Default)]
pub struct DrawCollider;

#[derive(Clone, Copy, Component, Debug)]
pub struct DrawColliderShape(Entity);

pub fn draw_colliders(
    mut commands: Commands,
    query: Query<(Entity, &Collider, &Colliding), Added<DrawCollider>>,
) {
    for (entity, collider, colliding) in query.iter() {
        let color = if colliding.0.is_empty() {
            Color::GREEN
        } else {
            Color::RED
        };

        let mut e = commands.spawn(GeometryBuilder::build_as(
            collider,
            DrawMode::Stroke(StrokeMode::color(color)),
            Transform::default(),
        ));
        e.set_parent(entity);

        let id = e.id();
        commands.entity(entity).insert(DrawColliderShape(id));
    }
}

pub fn update_colliders(
    mut commands: Commands,
    query: Query<(&Collider, &Colliding, &DrawColliderShape), Changed<Collider>>,
) {
    for (collider, colliding, DrawColliderShape(entity)) in query.iter() {
        let color = if colliding.0.is_empty() {
            Color::GREEN
        } else {
            Color::RED
        };

        commands.entity(*entity).insert(GeometryBuilder::build_as(
            collider,
            DrawMode::Stroke(StrokeMode::color(color)),
            Transform::default(),
        ));
    }
}

pub fn update_colors(
    collider_query: Query<(&Colliding, &DrawColliderShape), Changed<Colliding>>,
    mut shape_query: Query<&mut DrawMode>,
) {
    for (colliding, DrawColliderShape(entity)) in collider_query.iter() {
        let color = if colliding.0.is_empty() {
            Color::GREEN
        } else {
            Color::RED
        };

        let mut draw_mode = shape_query.get_mut(*entity).unwrap();
        *draw_mode = DrawMode::Stroke(StrokeMode::color(color));
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
