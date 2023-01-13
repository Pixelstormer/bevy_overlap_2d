use super::*;
use crate::transform_ext::TransformPoint2;
use bevy_prototype_lyon::prelude::tess::path::Winding;
use std::f32::consts::{PI, TAU};

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Circle {
    pub position: Vec2,
    pub radius: f32,
}

impl Circle {
    pub fn new(position: Vec2, radius: f32) -> Self {
        Self { position, radius }
    }

    pub fn from_diameter(position: Vec2, diameter: f32) -> Self {
        Self {
            position,
            radius: diameter / 2.0,
        }
    }

    pub fn radius_squared(&self) -> f32 {
        self.radius * self.radius
    }

    pub fn diameter(&self) -> f32 {
        self.radius * 2.0
    }

    pub fn circumference(&self) -> f32 {
        self.radius * TAU
    }

    pub fn area(&self) -> f32 {
        self.radius_squared() * PI
    }

    pub fn contains(&self, point: Vec2) -> bool {
        self.position.distance_squared(point) <= self.radius_squared()
    }

    pub fn support_point(&self, direction: Vec2) -> Vec2 {
        debug_assert!(direction.is_normalized());
        self.position + (direction * self.radius)
    }
}

impl Transformable for Circle {
    fn to_transformed(&self, transform: &GlobalTransform) -> Self {
        Self::new(transform.transform_point2(self.position), self.radius)
    }
}

impl Collides<Capsule> for Circle {
    fn collide(&self, other: &Capsule) -> Option<ContactManifold> {
        algorithms::collide_capsule_circle(other, self).neg()
    }
}

impl Collides<Circle> for Circle {
    fn collide(&self, other: &Circle) -> Option<ContactManifold> {
        algorithms::collide_circle_circle(self, other)
    }
}

impl Collides<Line> for Circle {
    fn collide(&self, other: &Line) -> Option<ContactManifold> {
        (other.distance_to_point_squared(&self.position) <= self.radius_squared()).into()
    }
}

impl Collides<Point> for Circle {
    fn collide(&self, other: &Point) -> Option<ContactManifold> {
        algorithms::collide_circle_point(self, other)
    }
}

impl Collides<Polygon> for Circle {
    fn collide(&self, other: &Polygon) -> Option<ContactManifold> {
        other.collide(self)
    }
}

impl Collides<Rectangle> for Circle {
    fn collide(&self, other: &Rectangle) -> Option<ContactManifold> {
        algorithms::collide_circle_rect(self, other)
    }
}

impl Collides<Triangle> for Circle {
    fn collide(&self, other: &Triangle) -> Option<ContactManifold> {
        todo!()
    }
}

#[cfg(feature = "debug-draw")]
impl Geometry for Circle {
    fn add_geometry(&self, b: &mut Builder) {
        b.add_circle((0.0, 0.0).into(), self.radius, Winding::Positive);
    }
}
