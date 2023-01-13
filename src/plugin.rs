use super::{
    collider::*,
    draw::DrawPlugin,
    layers::{CollisionLayers, CollisionLayersLabel},
};
use bevy::prelude::*;
use std::ops::Neg;

#[derive(StageLabel)]
pub struct CollisionStage;

#[derive(SystemLabel)]
pub struct FindCollidingPairs;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>();

        #[cfg(feature = "debug-draw")]
        app.add_plugin(DrawPlugin);

        app.add_stage_after(
            CoreStage::Update,
            CollisionStage,
            SystemStage::single_threaded(),
        )
        .add_system_to_stage(
            CollisionStage,
            find_colliding_pairs.label(FindCollidingPairs),
        );
    }
}

#[derive(Clone, Copy, Debug)]
pub enum CollisionEvent {
    Began(CollisionBegan),
    Ended(CollisionEnded),
}

impl CollisionEvent {
    pub fn began(us: Entity, them: Entity, contact: ContactManifold) -> Self {
        Self::Began(CollisionBegan { us, them, contact })
    }

    pub fn ended(us: Entity, them: Entity) -> Self {
        Self::Ended(CollisionEnded { us, them })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CollisionBegan {
    pub us: Entity,
    pub them: Entity,
    pub contact: ContactManifold,
}

#[derive(Clone, Copy, Debug)]
pub struct CollisionEnded {
    pub us: Entity,
    pub them: Entity,
}

#[derive(Bundle, Default, Debug)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub colliding: Colliding,
    pub layers: CollisionLayers,
    pub transform: TransformBundle,
}

impl ColliderBundle {
    pub fn capsule(height: f32, radius: f32) -> Self {
        let half_height = height / 2.0;
        Self {
            collider: Capsule::new(
                Line::new((0.0, half_height).into(), (0.0, -half_height).into()),
                radius,
            )
            .into(),
            ..Default::default()
        }
    }

    pub fn circle(radius: f32) -> Self {
        Self {
            collider: Circle::new(Vec2::ZERO, radius).into(),
            ..Default::default()
        }
    }

    // pub fn line(start: Vec2, end: Vec2) -> Self {
    //     Self {
    //         collider: Line::new(start, end).into(),
    //         ..Default::default()
    //     }
    // }

    pub fn point(point: Vec2) -> Self {
        Self {
            collider: Point::new(point).into(),
            ..Default::default()
        }
    }

    pub fn polygon(points: impl Into<Vec<Vec2>>) -> Self {
        Self {
            collider: Polygon::new(points.into()).into(),
            ..Default::default()
        }
    }

    pub fn rect(rect: Rect) -> Self {
        Self {
            collider: rect.into(),
            ..Default::default()
        }
    }

    // pub fn triangle(a: Vec2, b: Vec2, c: Vec2) -> Self {
    //     Self {
    //         collider: Triangle::new(a, b, c).into(),
    //         ..Default::default()
    //     }
    // }

    /// Specifies that this collider will only interact with other colliders that have matching layers,
    /// including colliders that have no specified layer
    pub fn with_layers_inclusive(mut self, layers: impl CollisionLayersLabel) -> Self {
        self.layers = CollisionLayers::Inclusive(layers.into_layers());
        self
    }

    /// Specifies that this collider will only interact with other colliders that have matching layers,
    /// excluding colliders that have no specified layer
    pub fn with_layers_exclusive(mut self, layers: impl CollisionLayersLabel) -> Self {
        self.layers = CollisionLayers::Exclusive(layers.into_layers());
        self
    }
}

fn find_colliding_pairs(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &GlobalTransform,
        &Collider,
        &CollisionLayers,
        &mut Colliding,
    )>,
    mut events: EventWriter<CollisionEvent>,
) {
    let mut events_batch = Vec::new();

    let mut iter = query.iter_combinations_mut();
    while let Some([us, them]) = iter.fetch_next() {
        let (us_entity, us_transform, us_collider, us_layers, mut us_colliding) = us;
        let (them_entity, them_transform, them_collider, them_layers, mut them_colliding) = them;

        if !us_layers.intersects(them_layers) {
            continue;
        }

        let contact = us_collider
            .to_transformed(us_transform)
            .collide(&them_collider.to_transformed(them_transform));

        if let Some(manifold) = contact {
            let us_was_disjoint = us_colliding.0.insert(them_entity, manifold).is_none();
            let them_was_disjoint = them_colliding.0.insert(us_entity, manifold.neg()).is_none();
            if us_was_disjoint && them_was_disjoint {
                // Only send a collision event if neither entity was already colliding with the other
                events_batch.push(CollisionEvent::began(us_entity, them_entity, manifold));
            }
        } else {
            let us_was_colliding = us_colliding.0.remove(&them_entity).is_some();
            let them_was_colliding = them_colliding.0.remove(&us_entity).is_some();
            if us_was_colliding && them_was_colliding {
                // Only send a collision event if both entities were previously colliding with eachother
                events_batch.push(CollisionEvent::ended(us_entity, them_entity));
            }
        }
    }

    events.send_batch(events_batch);
}
