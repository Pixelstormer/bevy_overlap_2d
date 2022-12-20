mod capsule;
mod circle;
mod line;
mod point;
mod polygon;
mod rect;
mod triangle;

use bevy::prelude::{Component, GlobalTransform, Rect as BevyRect, Transform, Vec2};
pub use {
    super::transform_ext::TransformPoint2, capsule::Capsule, circle::Circle, line::Line,
    point::Point, polygon::Polygon, rect::Rect, triangle::Triangle,
};

pub trait Transformable {
    fn to_transformed(&self, transform: &GlobalTransform) -> Self;
}

pub trait Collides<T>: Transformable {
    fn collide(&self, other: &T) -> CollisionResult;
}

#[derive(Clone, Default, Debug)]
pub struct CollisionResult {
    pub colliding: bool,
    // pub penetration_depth: f32,
    // pub contact_normal: Vec2,
}

impl CollisionResult {
    pub fn new(colliding: bool) -> Self {
        CollisionResult { colliding }
    }
}

impl From<bool> for CollisionResult {
    fn from(colliding: bool) -> Self {
        Self { colliding }
    }
}

#[derive(Clone, Component, Debug)]
pub enum Collider {
    Capsule(Capsule),
    Circle(Circle),
    Line(Line),
    Point(Point),
    Polygon(Polygon),
    Rect(Rect),
    Triangle(Triangle),
}

impl Collider {
    pub fn new(collider: impl Into<Self>) -> Self {
        collider.into()
    }
}

impl Default for Collider {
    fn default() -> Self {
        Self::Point(Point::default())
    }
}

impl From<Capsule> for Collider {
    fn from(capsule: Capsule) -> Self {
        Self::Capsule(capsule)
    }
}

impl From<Circle> for Collider {
    fn from(circle: Circle) -> Self {
        Self::Circle(circle)
    }
}

impl From<Line> for Collider {
    fn from(line: Line) -> Self {
        Self::Line(line)
    }
}

impl From<Point> for Collider {
    fn from(point: Point) -> Self {
        Self::Point(point)
    }
}

impl From<Vec2> for Collider {
    fn from(point: Vec2) -> Self {
        Self::Point(point.into())
    }
}

impl From<Polygon> for Collider {
    fn from(polygon: Polygon) -> Self {
        Self::Polygon(polygon)
    }
}

impl From<Rect> for Collider {
    fn from(rect: Rect) -> Self {
        Self::Rect(rect)
    }
}

impl From<BevyRect> for Collider {
    fn from(rect: BevyRect) -> Self {
        Self::Rect(rect.into())
    }
}

impl From<Triangle> for Collider {
    fn from(triangle: Triangle) -> Self {
        Self::Triangle(triangle)
    }
}

impl Transformable for Collider {
    fn to_transformed(&self, transform: &GlobalTransform) -> Self {
        match self {
            Collider::Capsule(shape) => shape.to_transformed(transform).into(),
            Collider::Circle(shape) => shape.to_transformed(transform).into(),
            Collider::Line(shape) => shape.to_transformed(transform).into(),
            Collider::Point(shape) => shape.to_transformed(transform).into(),
            Collider::Polygon(shape) => shape.to_transformed(transform).into(),
            Collider::Rect(shape) => shape.to_transformed(transform).into(),
            Collider::Triangle(shape) => shape.to_transformed(transform).into(),
        }
    }
}

impl Collides<Collider> for Collider {
    fn collide(&self, other: &Collider) -> CollisionResult {
        match (self, other) {
            (Collider::Capsule(a), Collider::Capsule(b)) => a.collide(b),
            (Collider::Capsule(a), Collider::Circle(b)) => a.collide(b),
            (Collider::Capsule(a), Collider::Line(b)) => a.collide(b),
            (Collider::Capsule(a), Collider::Point(b)) => a.collide(b),
            (Collider::Capsule(a), Collider::Polygon(b)) => a.collide(b),
            (Collider::Capsule(a), Collider::Rect(b)) => a.collide(b),
            (Collider::Capsule(a), Collider::Triangle(b)) => a.collide(b),
            (Collider::Circle(a), Collider::Capsule(b)) => a.collide(b),
            (Collider::Circle(a), Collider::Circle(b)) => a.collide(b),
            (Collider::Circle(a), Collider::Line(b)) => a.collide(b),
            (Collider::Circle(a), Collider::Point(b)) => a.collide(b),
            (Collider::Circle(a), Collider::Polygon(b)) => a.collide(b),
            (Collider::Circle(a), Collider::Rect(b)) => a.collide(b),
            (Collider::Circle(a), Collider::Triangle(b)) => a.collide(b),
            (Collider::Line(a), Collider::Capsule(b)) => a.collide(b),
            (Collider::Line(a), Collider::Circle(b)) => a.collide(b),
            (Collider::Line(a), Collider::Line(b)) => a.collide(b),
            (Collider::Line(a), Collider::Point(b)) => a.collide(b),
            (Collider::Line(a), Collider::Polygon(b)) => a.collide(b),
            (Collider::Line(a), Collider::Rect(b)) => a.collide(b),
            (Collider::Line(a), Collider::Triangle(b)) => a.collide(b),
            (Collider::Point(a), Collider::Capsule(b)) => a.collide(b),
            (Collider::Point(a), Collider::Circle(b)) => a.collide(b),
            (Collider::Point(a), Collider::Line(b)) => a.collide(b),
            (Collider::Point(a), Collider::Point(b)) => a.collide(b),
            (Collider::Point(a), Collider::Polygon(b)) => a.collide(b),
            (Collider::Point(a), Collider::Rect(b)) => a.collide(b),
            (Collider::Point(a), Collider::Triangle(b)) => a.collide(b),
            (Collider::Polygon(a), Collider::Capsule(b)) => a.collide(b),
            (Collider::Polygon(a), Collider::Circle(b)) => a.collide(b),
            (Collider::Polygon(a), Collider::Line(b)) => a.collide(b),
            (Collider::Polygon(a), Collider::Point(b)) => a.collide(b),
            (Collider::Polygon(a), Collider::Polygon(b)) => a.collide(b),
            (Collider::Polygon(a), Collider::Rect(b)) => a.collide(b),
            (Collider::Polygon(a), Collider::Triangle(b)) => a.collide(b),
            (Collider::Rect(a), Collider::Capsule(b)) => a.collide(b),
            (Collider::Rect(a), Collider::Circle(b)) => a.collide(b),
            (Collider::Rect(a), Collider::Line(b)) => a.collide(b),
            (Collider::Rect(a), Collider::Point(b)) => a.collide(b),
            (Collider::Rect(a), Collider::Polygon(b)) => a.collide(b),
            (Collider::Rect(a), Collider::Rect(b)) => a.collide(b),
            (Collider::Rect(a), Collider::Triangle(b)) => a.collide(b),
            (Collider::Triangle(a), Collider::Capsule(b)) => a.collide(b),
            (Collider::Triangle(a), Collider::Circle(b)) => a.collide(b),
            (Collider::Triangle(a), Collider::Line(b)) => a.collide(b),
            (Collider::Triangle(a), Collider::Point(b)) => a.collide(b),
            (Collider::Triangle(a), Collider::Polygon(b)) => a.collide(b),
            (Collider::Triangle(a), Collider::Rect(b)) => a.collide(b),
            (Collider::Triangle(a), Collider::Triangle(b)) => a.collide(b),
        }
    }
}

#[cfg(feature = "debug-draw")]
impl bevy_prototype_lyon::geometry::Geometry for Collider {
    fn add_geometry(&self, b: &mut bevy_prototype_lyon::prelude::tess::path::path::Builder) {
        match self {
            Collider::Capsule(shape) => shape.add_geometry(b),
            Collider::Circle(shape) => shape.add_geometry(b),
            Collider::Line(shape) => shape.add_geometry(b),
            Collider::Point(shape) => shape.add_geometry(b),
            Collider::Polygon(shape) => shape.add_geometry(b),
            Collider::Rect(shape) => shape.add_geometry(b),
            Collider::Triangle(shape) => shape.add_geometry(b),
        }
    }
}
