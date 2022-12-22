use super::*;
use bevy::prelude::{Rect as BevyRect, Vec2};
use bevy_prototype_lyon::prelude::tess::geom::Box2D;

#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub struct Rect(pub BevyRect);

impl Rect {
    pub fn new(rect: BevyRect) -> Self {
        Self(rect)
    }

    pub fn from_corners(p0: Vec2, p1: Vec2) -> Self {
        Self(BevyRect {
            min: p0.min(p1),
            max: p0.max(p1),
        })
    }

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

    pub fn top_corner(&self) -> Vec2 {
        Vec2::new(self.0.min.x, self.0.max.y)
    }

    pub fn bottom_corner(&self) -> Vec2 {
        Vec2::new(self.0.max.x, self.0.min.y)
    }

    pub fn left(&self) -> Line {
        Line::new(self.0.min, self.top_corner())
    }

    pub fn top(&self) -> Line {
        Line::new(self.top_corner(), self.0.max)
    }

    pub fn right(&self) -> Line {
        Line::new(self.bottom_corner(), self.0.max)
    }

    pub fn bottom(&self) -> Line {
        Line::new(self.0.min, self.bottom_corner())
    }

    pub fn contains(&self, point: Vec2) -> bool {
        self.0.contains(point)
    }
}

impl From<BevyRect> for Rect {
    fn from(rect: BevyRect) -> Self {
        Self(rect)
    }
}

impl Transformable for Rect {
    fn to_transformed(&self, transform: &GlobalTransform) -> Self {
        Self::new(BevyRect {
            min: transform.transform_point2(self.0.min),
            max: transform.transform_point2(self.0.max),
        })
    }
}

impl Collides<Capsule> for Rect {
    fn collide(&self, other: &Capsule) -> CollisionResult {
        other.collide(self)
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
        other.collide(self)
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
