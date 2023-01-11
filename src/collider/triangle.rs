use super::*;
use crate::transform_ext::TransformPoint2;

/// A triangle.
#[derive(Clone, Copy, Default, Debug, PartialEq)]
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

impl Transformable for Triangle {
    fn to_transformed(&self, transform: &GlobalTransform) -> Self {
        Self::new(
            transform.transform_point2(self.a),
            transform.transform_point2(self.b),
            transform.transform_point2(self.c),
        )
    }
}

impl Collides<Capsule> for Triangle {
    fn collide(&self, other: &Capsule) -> ContactManifold {
        todo!()
    }
}

impl Collides<Circle> for Triangle {
    fn collide(&self, other: &Circle) -> ContactManifold {
        todo!()
    }
}

impl Collides<Line> for Triangle {
    fn collide(&self, other: &Line) -> ContactManifold {
        todo!()
    }
}

impl Collides<Point> for Triangle {
    fn collide(&self, other: &Point) -> ContactManifold {
        todo!()
    }
}

impl Collides<Polygon> for Triangle {
    fn collide(&self, other: &Polygon) -> ContactManifold {
        todo!()
    }
}

impl Collides<Rectangle> for Triangle {
    fn collide(&self, other: &Rectangle) -> ContactManifold {
        todo!()
    }
}

impl Collides<Triangle> for Triangle {
    fn collide(&self, other: &Triangle) -> ContactManifold {
        todo!()
    }
}

#[cfg(feature = "debug-draw")]
impl Geometry for Triangle {
    fn add_geometry(&self, b: &mut Builder) {
        b.begin((self.a.x, self.a.y).into());
        b.line_to((self.b.x, self.b.y).into());
        b.line_to((self.c.x, self.c.y).into());
        b.end(true);
    }
}
