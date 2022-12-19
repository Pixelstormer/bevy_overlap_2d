use super::*;
use bevy::prelude::Vec2;

/// A triangle.
#[derive(Clone, Copy, Default)]
pub struct Triangle {
    pub a: Vec2,
    pub b: Vec2,
    pub c: Vec2,
}

impl Triangle {
    pub fn new(a: Vec2, b: Vec2, c: Vec2) -> Self {
        Self { a, b, c }
    }

    pub fn from_array(arr: [Vec2; 3]) -> Self {
        Self {
            a: arr[0],
            b: arr[1],
            c: arr[2],
        }
    }

    pub fn area(&self) -> f32 {
        0.5 * (self.b - self.a).perp_dot(self.c - self.a).abs()
    }

    pub fn perimeter(&self) -> f32 {
        self.a.distance(self.b) + self.a.distance(self.c) + self.b.distance(self.c)
    }
}

impl Collides<Capsule> for Triangle {
    fn collide(&self, other: &Capsule) -> CollisionResult {
        todo!()
    }
}

impl Collides<Circle> for Triangle {
    fn collide(&self, other: &Circle) -> CollisionResult {
        todo!()
    }
}

impl Collides<Line> for Triangle {
    fn collide(&self, other: &Line) -> CollisionResult {
        todo!()
    }
}

impl Collides<Point> for Triangle {
    fn collide(&self, other: &Point) -> CollisionResult {
        todo!()
    }
}

impl Collides<Polygon> for Triangle {
    fn collide(&self, other: &Polygon) -> CollisionResult {
        todo!()
    }
}

impl Collides<Rect> for Triangle {
    fn collide(&self, other: &Rect) -> CollisionResult {
        todo!()
    }
}

impl Collides<Triangle> for Triangle {
    fn collide(&self, other: &Triangle) -> CollisionResult {
        todo!()
    }
}

#[cfg(feature = "debug-draw")]
impl bevy_prototype_lyon::geometry::Geometry for Triangle {
    fn add_geometry(&self, b: &mut bevy_prototype_lyon::prelude::tess::path::path::Builder) {
        b.begin((self.a.x, self.a.y).into());
        b.line_to((self.b.x, self.b.y).into());
        b.line_to((self.c.x, self.c.y).into());
        b.end(true);
    }
}
