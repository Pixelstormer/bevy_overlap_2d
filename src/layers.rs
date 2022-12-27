use bevy::prelude::*;
use bitflags::bitflags;
use std::ops::BitOr;

pub trait CollisionLayersLabel {
    fn into_layers(self) -> CollisionLayerFlags;
}

bitflags! {
    pub struct CollisionLayerFlags: u64 {
        const EVERYTHING = u64::MAX;
    }
}

impl CollisionLayersLabel for CollisionLayerFlags {
    fn into_layers(self) -> CollisionLayerFlags {
        self
    }
}

#[derive(Clone, Copy, Component, Debug, Default)]
pub enum CollisionLayers {
    /// Collides with colliders on the None and Inclusive layers
    #[default]
    None,
    /// Collides with colliders on the None layer, and those on the Inclusive and Exclusive layers that have
    /// intersecting flags
    Inclusive(CollisionLayerFlags),
    /// Collides with colliders on the Inclusive and Exclusive layers that have intersecting flags
    Exclusive(CollisionLayerFlags),
}

impl CollisionLayers {
    pub fn flags(&self) -> Option<&CollisionLayerFlags> {
        match self {
            CollisionLayers::None => None,
            CollisionLayers::Inclusive(flags) => Some(flags),
            CollisionLayers::Exclusive(flags) => Some(flags),
        }
    }

    pub fn intersects(&self, other: &CollisionLayers) -> bool {
        let Some(flags) = self.flags() else {
            return !matches!(other, CollisionLayers::Exclusive(_));
        };

        let Some(other) = other.flags() else {
            return matches!(self, CollisionLayers::Inclusive(_));
        };

        flags.intersects(*other)
    }
}
