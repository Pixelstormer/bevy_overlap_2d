use super::{
    collider::*,
    draw::DrawPlugin,
    layers::{CollisionLayers, CollisionLayersLabel},
};
use bevy::prelude::*;

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
    pub fn began(a: Entity, b: Entity) -> Self {
        Self::Began(CollisionBegan { a, b })
    }

    pub fn ended(a: Entity, b: Entity) -> Self {
        Self::Ended(CollisionEnded { a, b })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CollisionBegan {
    pub a: Entity,
    pub b: Entity,
}

#[derive(Clone, Copy, Debug)]
pub struct CollisionEnded {
    pub a: Entity,
    pub b: Entity,
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

    pub fn line(start: Vec2, end: Vec2) -> Self {
        Self {
            collider: Line::new(start, end).into(),
            ..Default::default()
        }
    }

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
            collider: Rectangle::new(rect).into(),
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
    let size = query.iter_combinations::<2>().size_hint().0;
    let mut events_batch = Vec::with_capacity(size);

    let mut iter = query.iter_combinations_mut();
    while let Some([a, b]) = iter.fetch_next() {
        let (a_entity, a_transform, a_collider, a_layers, mut a_colliding) = a;
        let (b_entity, b_transform, b_collider, b_layers, mut b_colliding) = b;

        if !a_layers.intersects(b_layers) {
            continue;
        }

        let result = a_collider
            .to_transformed(a_transform)
            .collide(&b_collider.to_transformed(b_transform));

        if result.colliding() {
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
