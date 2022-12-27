mod collider;
mod layers;
mod plugin;
mod transform_ext;

#[cfg(feature = "debug-draw")]
mod draw;

pub use bevy_overlap_2d_derive::CollisionLayersLabel;
pub use collider::*;
pub use layers::*;
pub use plugin::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        //let result = add(2, 2);
        //assert_eq!(result, 4);
    }
}
