use super::collider::{Collider, Colliding};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{DrawMode, GeometryBuilder, ShapePlugin, StrokeMode};

pub struct DrawPlugin;

impl Plugin for DrawPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ShapePlugin)
            .init_resource::<DrawColors>()
            .add_system(draw_colliders)
            .add_system(update_colliders)
            .add_system(update_colors)
            .add_system(undraw_colliders);
    }
}

#[derive(Bundle, Default, Debug)]
pub struct ColliderDrawBundle {
    pub draw: DrawCollider,
    pub visibility: VisibilityBundle,
}

#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct DrawColors {
    pub disjoint: DrawMode,
    pub colliding: DrawMode,
}

impl Default for DrawColors {
    fn default() -> Self {
        Self {
            disjoint: DrawMode::Stroke(StrokeMode::color(Color::GREEN)),
            colliding: DrawMode::Stroke(StrokeMode::color(Color::RED)),
        }
    }
}

#[derive(Clone, Copy, Component, Default, Debug)]
pub struct DrawCollider;

#[derive(Clone, Copy, Component, Debug, PartialEq, Eq)]
pub struct DrawColliderShape(Entity);

pub fn draw_colliders(
    mut commands: Commands,
    query: Query<(Entity, &Collider, &Colliding), Added<DrawCollider>>,
    colors: Res<DrawColors>,
) {
    for (entity, collider, colliding) in query.iter() {
        let color = if colliding.0.is_empty() {
            colors.disjoint
        } else {
            colors.colliding
        };

        let mut e = commands.spawn(GeometryBuilder::build_as(
            collider,
            color,
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
    colors: Res<DrawColors>,
) {
    for (collider, colliding, DrawColliderShape(entity)) in query.iter() {
        let color = if colliding.0.is_empty() {
            colors.disjoint
        } else {
            colors.colliding
        };

        commands.entity(*entity).insert(GeometryBuilder::build_as(
            collider,
            color,
            Transform::default(),
        ));
    }
}

pub fn update_colors(
    collider_query: Query<(&Colliding, &DrawColliderShape), Changed<Colliding>>,
    mut shape_query: Query<&mut DrawMode>,
    colors: Res<DrawColors>,
) {
    for (colliding, DrawColliderShape(entity)) in collider_query.iter() {
        let color = if colliding.0.is_empty() {
            colors.disjoint
        } else {
            colors.colliding
        };

        let mut draw_mode = shape_query.get_mut(*entity).unwrap();
        *draw_mode = color;
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
