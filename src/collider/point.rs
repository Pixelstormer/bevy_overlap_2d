use super::*;
use crate::transform_ext::TransformPoint2;
use bevy_prototype_lyon::prelude::tess::geom::LineSegment;

#[derive(Clone, Copy, PartialEq, Default, Debug)]
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
    fn collide(&self, other: &Capsule) -> CollisionResult {
        other.collide(self)
    }
}

impl Collides<Circle> for Point {
    fn collide(&self, other: &Circle) -> CollisionResult {
        other.collide(self)
    }
}

impl Collides<Line> for Point {
    fn collide(&self, other: &Line) -> CollisionResult {
        other.collide(self)
    }
}

impl Collides<Point> for Point {
    fn collide(&self, other: &Point) -> CollisionResult {
        (self == other).into()
    }
}

impl Collides<Polygon> for Point {
    fn collide(&self, other: &Polygon) -> CollisionResult {
        other.collide(self)
    }
}

impl Collides<Rectangle> for Point {
    fn collide(&self, other: &Rectangle) -> CollisionResult {
        other.collide(self)
    }
}

impl Collides<Triangle> for Point {
    fn collide(&self, other: &Triangle) -> CollisionResult {
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
