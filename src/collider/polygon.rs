use super::*;
use bevy::prelude::Vec2;

/// An arbitrary polygon.
#[derive(Clone, Default)]
pub struct Polygon {
    pub vertices: Vec<Vec2>,
}

impl Collides<Capsule> for Polygon {
    fn collide(&self, other: &Capsule) -> CollisionResult {
        todo!()
    }
}

impl Collides<Circle> for Polygon {
    fn collide(&self, other: &Circle) -> CollisionResult {
        todo!()
    }
}

impl Collides<Line> for Polygon {
    fn collide(&self, other: &Line) -> CollisionResult {
        todo!()
    }
}

impl Collides<Point> for Polygon {
    fn collide(&self, other: &Point) -> CollisionResult {
        todo!()
    }
}

impl Collides<Polygon> for Polygon {
    fn collide(&self, other: &Polygon) -> CollisionResult {
        todo!()
    }
}

impl Collides<Rect> for Polygon {
    fn collide(&self, other: &Rect) -> CollisionResult {
        todo!()
    }
}

impl Collides<Triangle> for Polygon {
    fn collide(&self, other: &Triangle) -> CollisionResult {
        todo!()
    }
}

#[cfg(feature = "debug-draw")]
impl bevy_prototype_lyon::geometry::Geometry for Polygon {
    fn add_geometry(&self, b: &mut bevy_prototype_lyon::prelude::tess::path::path::Builder) {
        let mut iter = self.vertices.iter();

        let first = if let Some(first) = iter.next() {
            first
        } else {
            return;
        };

        b.begin((first.x, first.y).into());

        for vertex in iter {
            b.line_to((vertex.x, vertex.y).into());
        }

        b.end(true);
    }
}
