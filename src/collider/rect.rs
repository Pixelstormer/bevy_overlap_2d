use super::*;
use crate::transform_ext::TransformPoint2;
use bevy_prototype_lyon::prelude::tess::{geom::Box2D, path::Winding};

#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct Rectangle(pub Rect);

impl Rectangle {
    pub fn new(rect: Rect) -> Self {
        Self(rect)
    }

    pub fn from_corners(p0: Vec2, p1: Vec2) -> Self {
        Self(Rect {
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
        Vec2::new(self.min().x, self.max().y)
    }

    pub fn bottom_corner(&self) -> Vec2 {
        Vec2::new(self.max().x, self.min().y)
    }

    pub fn left(&self) -> Line {
        Line::new(self.min(), self.top_corner())
    }

    pub fn top(&self) -> Line {
        Line::new(self.top_corner(), self.max())
    }

    pub fn right(&self) -> Line {
        Line::new(self.bottom_corner(), self.max())
    }

    pub fn bottom(&self) -> Line {
        Line::new(self.min(), self.bottom_corner())
    }

    pub fn contains(&self, point: Vec2) -> bool {
        self.0.contains(point)
    }

    /// The closest point within the bounds of `self` to the given point.
    pub fn closest_point(&self, point: Vec2) -> Vec2 {
        point.clamp(self.min(), self.max())
    }

    /// The closest point on the perimeter of `self` to the given point, as well as a boolean indicating
    /// whether the given point lies inside `self` or not.
    pub fn closest_point_on_perimeter(&self, point: Vec2) -> (Vec2, bool) {
        let closest_point = self.closest_point(point);
        let point_is_in_rect = closest_point == point;

        point_is_in_rect
            .then(|| {
                let left = point.x - self.min().x;
                let top = self.max().y - point.y;
                let right = self.max().x - point.x;
                let bottom = point.y - self.min().y;

                if left <= top && left <= right && left <= bottom {
                    Vec2::new(point.x - left, point.y)
                } else if top <= right && top <= bottom {
                    Vec2::new(point.x, point.y + top)
                } else if right <= bottom {
                    Vec2::new(point.x + right, point.y)
                } else {
                    Vec2::new(point.x, point.y - bottom)
                }
            })
            .map_or((closest_point, false), |point| (point, true))
    }
}

impl From<Rect> for Rectangle {
    fn from(rect: Rect) -> Self {
        Self(rect)
    }
}

impl Transformable for Rectangle {
    fn to_transformed(&self, transform: &GlobalTransform) -> Self {
        Self::new(Rect {
            min: transform.transform_point2(self.min()),
            max: transform.transform_point2(self.max()),
        })
    }
}

impl Collides<Capsule> for Rectangle {
    fn collide(&self, other: &Capsule) -> ContactManifold {
        other.collide(self)
    }
}

impl Collides<Circle> for Rectangle {
    fn collide(&self, other: &Circle) -> ContactManifold {
        algorithms::collide_rect_circle(self, other)
    }
}

impl Collides<Line> for Rectangle {
    fn collide(&self, other: &Line) -> ContactManifold {
        other.collide(self)
    }
}

impl Collides<Point> for Rectangle {
    fn collide(&self, other: &Point) -> ContactManifold {
        self.contains(other.0).into()
    }
}

impl Collides<Polygon> for Rectangle {
    fn collide(&self, other: &Polygon) -> ContactManifold {
        other.collide(self)
    }
}

impl Collides<Rectangle> for Rectangle {
    fn collide(&self, other: &Rectangle) -> ContactManifold {
        algorithms::collide_rect_rect(self, other)
    }
}

impl Collides<Triangle> for Rectangle {
    fn collide(&self, other: &Triangle) -> ContactManifold {
        todo!()
    }
}

#[cfg(feature = "debug-draw")]
impl Geometry for Rectangle {
    fn add_geometry(&self, b: &mut Builder) {
        b.add_rectangle(
            &Box2D::new(
                (self.min().x, self.min().y).into(),
                (self.max().x, self.max().y).into(),
            ),
            Winding::Positive,
        );
    }
}
