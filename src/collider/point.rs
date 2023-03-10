use super::*;
use crate::transform_ext::TransformPoint2;
use bevy_prototype_lyon::prelude::tess::geom::LineSegment;

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Point(pub Vec2);

impl Point {
    pub fn new(point: Vec2) -> Self {
        Self(point)
    }

    pub fn distance(&self, other: Vec2) -> f32 {
        self.0.distance(other)
    }

    pub fn distance_squared(&self, other: Vec2) -> f32 {
        self.0.distance_squared(other)
    }
}

impl From<Vec2> for Point {
    fn from(point: Vec2) -> Self {
        Self(point)
    }
}

impl Transformable for Point {
    fn to_transformed(&self, transform: &GlobalTransform) -> Self {
        Self::new(transform.transform_point2(self.0))
    }
}

impl Collides<Capsule> for Point {
    fn collide(&self, other: &Capsule) -> Option<ContactManifold> {
        algorithms::collide_capsule_point(other, self).neg()
    }
}

impl Collides<Circle> for Point {
    fn collide(&self, other: &Circle) -> Option<ContactManifold> {
        algorithms::collide_circle_point(other, self).neg()
    }
}

impl Collides<Line> for Point {
    fn collide(&self, other: &Line) -> Option<ContactManifold> {
        other.collide(self)
    }
}

impl Collides<Point> for Point {
    fn collide(&self, other: &Point) -> Option<ContactManifold> {
        algorithms::collide_point_point(self, other)
    }
}

impl Collides<Polygon> for Point {
    fn collide(&self, other: &Polygon) -> Option<ContactManifold> {
        other.collide(self)
    }
}

impl Collides<Rectangle> for Point {
    fn collide(&self, other: &Rectangle) -> Option<ContactManifold> {
        algorithms::collide_point_rect(self, other)
    }
}

impl Collides<Triangle> for Point {
    fn collide(&self, other: &Triangle) -> Option<ContactManifold> {
        todo!()
    }
}

#[cfg(feature = "debug-draw")]
impl Geometry for Point {
    fn add_geometry(&self, b: &mut Builder) {
        b.add_line_segment(&LineSegment {
            from: (self.0.x - 5.0, self.0.y - 5.0).into(),
            to: (self.0.x + 5.0, self.0.y + 5.0).into(),
        });

        b.add_line_segment(&LineSegment {
            from: (self.0.x + 5.0, self.0.y - 5.0).into(),
            to: (self.0.x - 5.0, self.0.y + 5.0).into(),
        });
    }
}
