use super::*;
use bevy_prototype_lyon::prelude::tess::{
    geom::Box2D,
    path::{builder::BorderRadii, Winding},
};
use std::f32::consts::PI;

/// A 2d capsule is a shape that can be imagined as a rectangle with a pair of semicircles attached to
/// opposite sides, or more formally, the set of all points within a certain distance from a line.
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Capsule {
    pub line: Line,
    pub radius: f32,
}

impl Capsule {
    pub fn new(line: Line, radius: f32) -> Self {
        Self { line, radius }
    }

    pub fn radius_squared(&self) -> f32 {
        self.radius * self.radius
    }

    pub fn perimeter(&self) -> f32 {
        2.0 * ((PI * self.radius) + self.line.length())
    }
}

impl Transformable for Capsule {
    fn to_transformed(&self, transform: &GlobalTransform) -> Self {
        Self::new(self.line.to_transformed(transform), self.radius)
    }
}

impl Collides<Capsule> for Capsule {
    fn collide(&self, other: &Capsule) -> Option<ContactManifold> {
        algorithms::collide_capsule_capsule(self, other)
    }
}

impl Collides<Circle> for Capsule {
    fn collide(&self, other: &Circle) -> Option<ContactManifold> {
        algorithms::collide_capsule_circle(self, other)
    }
}

impl Collides<Line> for Capsule {
    fn collide(&self, other: &Line) -> Option<ContactManifold> {
        (self.line.closest_point_to_line(other) <= self.radius_squared()).into()
    }
}

impl Collides<Point> for Capsule {
    fn collide(&self, other: &Point) -> Option<ContactManifold> {
        algorithms::collide_capsule_point(self, other)
    }
}

impl Collides<Polygon> for Capsule {
    fn collide(&self, other: &Polygon) -> Option<ContactManifold> {
        other.collide(self)
    }
}

impl Collides<Rectangle> for Capsule {
    fn collide(&self, other: &Rectangle) -> Option<ContactManifold> {
        (other.contains(self.line.start)
            || other.contains(self.line.end)
            || self.collide(&other.left()).colliding
            || self.collide(&other.top()).colliding
            || self.collide(&other.right()).colliding
            || self.collide(&other.bottom()).colliding)
            .into()
    }
}

impl Collides<Triangle> for Capsule {
    fn collide(&self, other: &Triangle) -> Option<ContactManifold> {
        todo!()
    }
}

#[cfg(feature = "debug-draw")]
impl Geometry for Capsule {
    fn add_geometry(&self, b: &mut Builder) {
        let rect = Rect::from_corners(self.line.start, self.line.end).inset(self.radius);
        b.add_rounded_rectangle(
            &Box2D::new(
                (rect.min.x, rect.min.y).into(),
                (rect.max.x, rect.max.y).into(),
            ),
            &BorderRadii::new(self.radius),
            Winding::Positive,
        );
    }
}
