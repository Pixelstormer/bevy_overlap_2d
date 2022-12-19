use super::collider::{Collider, Collides, CollisionResult};
use super::draw::draw_colliders;
use crate::draw::{undraw_colliders, update_colliders, DrawCollider};
use bevy::{prelude::*, render::render_phase::Draw, utils::HashSet};
use bevy_prototype_lyon::prelude::ShapePlugin;

#[derive(Component, Default)]
pub struct Colliding(pub HashSet<Entity>);

#[derive(StageLabel)]
pub struct CollisionStage;

#[derive(Clone)]
pub enum CollisionEvent {
    Began(CollisionBegan),
    Ended(CollisionEnded),
}

impl CollisionEvent {
    pub fn began(a: Entity, b: Entity) -> Self {
        Self::Began(CollisionBegan { a, b })
    }

    pub fn ended(a: Entity, b: Entity) -> Self {
        Self::Ended(CollisionEnded { a, b })
    }
}

#[derive(Clone)]
pub struct CollisionBegan {
    pub a: Entity,
    pub b: Entity,
}

#[derive(Clone)]
pub struct CollisionEnded {
    pub a: Entity,
    pub b: Entity,
}

#[derive(Bundle, Default)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub colliding: Colliding,
    pub transform: TransformBundle,
}

#[derive(Bundle, Default)]
pub struct ColliderDrawBundle {
    pub draw: DrawCollider,
    pub visibility: VisibilityBundle,
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<CollisionEvent>();

        #[cfg(feature = "debug-draw")]
        {
            app.add_plugin(ShapePlugin);
            app.add_system(draw_colliders);
            app.add_system(update_colliders);
            app.add_system(undraw_colliders);
        }

        app.add_stage_after(
            CoreStage::Update,
            CollisionStage,
            SystemStage::single_threaded(),
        );

        app.add_system_to_stage(CollisionStage, find_colliding_pairs);
    }
}

fn find_colliding_pairs(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, &Collider, &mut Colliding)>,
    mut events: EventWriter<CollisionEvent>,
) {
    let size = query.iter_combinations::<2>().size_hint().0;
    let mut events_batch = Vec::with_capacity(size);

    let mut iter = query.iter_combinations_mut();
    while let Some([a, b]) = iter.fetch_next() {
        let (a_entity, a_transform, a_collider, mut a_colliding) = a;
        let (b_entity, b_transform, b_collider, mut b_colliding) = b;

        let result = a_collider.collide(b_collider);

        if result.colliding {
            let a_was_disjoint = a_colliding.0.insert(b_entity);
            let b_was_disjoint = b_colliding.0.insert(a_entity);
            if a_was_disjoint && b_was_disjoint {
                // Only send a collision event if neither entity was already colliding with the other
                events_batch.push(CollisionEvent::began(a_entity, b_entity));
            }
        } else {
            let a_was_colliding = a_colliding.0.remove(&b_entity);
            let b_was_colliding = b_colliding.0.remove(&a_entity);
            if a_was_colliding && b_was_colliding {
                // Only send a collision event if both entities were previously colliding with eachother
                events_batch.push(CollisionEvent::ended(a_entity, b_entity));
            }
        }
    }

    events.send_batch(events_batch);
}
