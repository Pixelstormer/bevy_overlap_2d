use super::*;
use bevy::prelude::Vec2;

#[derive(Clone, Copy, PartialEq, Default)]
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

impl Collides<Capsule> for Point {
    fn collide(&self, other: &Capsule) -> CollisionResult {
        todo!()
    }
}

impl Collides<Circle> for Point {
    fn collide(&self, other: &Circle) -> CollisionResult {
        todo!()
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
        todo!()
    }
}

impl Collides<Rect> for Point {
    fn collide(&self, other: &Rect) -> CollisionResult {
        other.collide(self)
    }
}

impl Collides<Triangle> for Point {
    fn collide(&self, other: &Triangle) -> CollisionResult {
        todo!()
    }
}

#[cfg(feature = "debug-draw")]
impl bevy_prototype_lyon::geometry::Geometry for Point {
    fn add_geometry(&self, b: &mut bevy_prototype_lyon::prelude::tess::path::path::Builder) {
        b.add_point((self.0.x, self.0.y).into());
    }
}
