pub mod collider;
pub mod layers;
pub mod plugin;
pub mod transform_ext;

#[cfg(feature = "debug-draw")]
pub mod draw;

pub mod prelude {
    pub use super::collider::{
        Capsule, Circle, Collider, Collides, Colliding, CollisionResult, Line, Point, Polygon,
        Rectangle, Transformable, Triangle,
    };
    pub use super::draw::{DrawCollider, DrawColliderShape, DrawColors};
    pub use super::layers::{CollisionLayerFlags, CollisionLayers, CollisionLayersLabel};
    pub use super::plugin::{
        ColliderBundle, ColliderDrawBundle, CollisionBegan, CollisionEnded, CollisionEvent,
        CollisionPlugin, CollisionStage,
    };
    pub use bevy_overlap_2d_derive::CollisionLayersLabel;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        //let result = add(2, 2);
        //assert_eq!(result, 4);
    }
}
