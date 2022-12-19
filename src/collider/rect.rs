use super::*;
use bevy::prelude::Vec2;
use bevy_prototype_lyon::prelude::tess::geom::Box2D;

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct Rect(pub bevy::prelude::Rect);

impl Rect {
    pub fn min(&self) -> Vec2 {
        self.0.min
    }

    pub fn max(&self) -> Vec2 {
        self.0.max
    }

    pub fn width(&self) -> f32 {
        self.0.width()
    }

    pub fn height(&self) -> f32 {
        self.0.height()
    }

    pub fn contains(&self, point: Vec2) -> bool {
        self.0.contains(point)
    }
}

impl Collides<Capsule> for Rect {
    fn collide(&self, other: &Capsule) -> CollisionResult {
        todo!()
    }
}

impl Collides<Circle> for Rect {
    fn collide(&self, other: &Circle) -> CollisionResult {
        other.collide(self)
    }
}

impl Collides<Line> for Rect {
    fn collide(&self, other: &Line) -> CollisionResult {
        other.collide(self)
    }
}

impl Collides<Point> for Rect {
    fn collide(&self, other: &Point) -> CollisionResult {
        self.contains(other.0).into()
    }
}

impl Collides<Polygon> for Rect {
    fn collide(&self, other: &Polygon) -> CollisionResult {
        todo!()
    }
}

impl Collides<Rect> for Rect {
    fn collide(&self, other: &Rect) -> CollisionResult {
        (self.min().x <= other.max().x
            && self.min().y <= other.max().y
            && self.max().x >= other.min().x
            && self.max().y >= other.min().y)
            .into()
    }
}

impl Collides<Triangle> for Rect {
    fn collide(&self, other: &Triangle) -> CollisionResult {
        todo!()
    }
}

#[cfg(feature = "debug-draw")]
impl bevy_prototype_lyon::geometry::Geometry for Rect {
    fn add_geometry(&self, b: &mut bevy_prototype_lyon::prelude::tess::path::path::Builder) {
        b.add_rectangle(
            &Box2D::from_size((self.width(), self.height()).into()),
            bevy_prototype_lyon::prelude::tess::path::Winding::Positive,
        );
    }
}
