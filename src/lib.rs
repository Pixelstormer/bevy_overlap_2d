mod collider;
mod plugin;
mod transform_ext;

#[cfg(feature = "debug-draw")]
mod draw;

pub use collider::{
    Capsule, Circle, Collider, Collides, CollisionResult, Line, Point, Polygon, Rect, Triangle,
};
pub use plugin::{
    ColliderBundle, ColliderDrawBundle, Colliding, CollisionBegan, CollisionEnded, CollisionEvent,
    CollisionPlugin, CollisionStage,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        //let result = add(2, 2);
        //assert_eq!(result, 4);
    }
}
