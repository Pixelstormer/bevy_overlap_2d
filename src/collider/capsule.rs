use super::*;
use bevy::prelude::Vec2;
use bevy_prototype_lyon::prelude::tess::{
    geom::{euclid::Size2D, Box2D},
    path::{builder::BorderRadii, Winding},
};
use std::f32::consts::PI;

/// A 2d capsule is a shape that can be imagined as a rectangle with a pair of semicircles attached to
/// opposite sides, or more formally, the set of all points within a certain distance from a line.
#[derive(Clone, Copy, Default)]
pub struct Capsule {
    pub line: Line,
    pub radius: f32,
}

impl Capsule {
    pub fn radius_squared(&self) -> f32 {
        self.radius * self.radius
    }

    pub fn perimeter(&self) -> f32 {
        2.0 * ((PI * self.radius) + self.line.length())
    }
}

impl Collides<Capsule> for Capsule {
    fn collide(&self, other: &Capsule) -> CollisionResult {
        (self.line.distance_to_line_squared(&other.line)
            <= self.radius_squared() + other.radius_squared())
        .into()
    }
}

impl Collides<Circle> for Capsule {
    fn collide(&self, other: &Circle) -> CollisionResult {
        (self.line.distance_to_point_squared(&other.position)
            <= self.radius_squared() + other.radius_squared())
        .into()
    }
}

impl Collides<Line> for Capsule {
    fn collide(&self, other: &Line) -> CollisionResult {
        (self.line.distance_to_line_squared(other) <= self.radius_squared()).into()
    }
}

impl Collides<Point> for Capsule {
    fn collide(&self, other: &Point) -> CollisionResult {
        (self.line.distance_to_point_squared(&other.0) <= self.radius_squared()).into()
    }
}

impl Collides<Polygon> for Capsule {
    fn collide(&self, other: &Polygon) -> CollisionResult {
        todo!()
    }
}

impl Collides<Rect> for Capsule {
    fn collide(&self, other: &Rect) -> CollisionResult {
        todo!()
    }
}

impl Collides<Triangle> for Capsule {
    fn collide(&self, other: &Triangle) -> CollisionResult {
        todo!()
    }
}

#[cfg(feature = "debug-draw")]
impl bevy_prototype_lyon::geometry::Geometry for Capsule {
    fn add_geometry(&self, b: &mut bevy_prototype_lyon::prelude::tess::path::path::Builder) {
        b.add_rounded_rectangle(
            &Box2D::from_size((self.radius * 2.0, self.line.length() + (self.radius * 2.0)).into()),
            &BorderRadii::new(self.radius),
            Winding::Positive,
        );
    }
}
