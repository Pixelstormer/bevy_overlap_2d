use super::*;
use crate::transform_ext::TransformPoint2;
use bevy_prototype_lyon::prelude::tess::geom::LineSegment;

/// A straight line connecting two points.
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Line {
    pub start: Vec2,
    pub end: Vec2,
}

impl Line {
    pub fn new(start: Vec2, end: Vec2) -> Self {
        Line { start, end }
    }

    pub fn as_difference(&self) -> Vec2 {
        self.end - self.start
    }

    pub fn length(&self) -> f32 {
        self.as_difference().length()
    }

    pub fn length_squared(&self) -> f32 {
        self.as_difference().length_squared()
    }

    pub fn distance_to_point(&self, point: &Vec2) -> f32 {
        self.distance_to_point_squared(point).sqrt()
    }

    pub fn distance_to_point_squared(&self, point: &Vec2) -> f32 {
        let difference = self.as_difference();
        let length = difference.length_squared();
        if length == 0.0 {
            return point.distance_squared(self.start);
        }

        let t = ((*point - self.start).dot(difference) / length).clamp(0.0, 1.0);
        let projection = (difference * t) + self.start;
        point.distance_squared(projection)
    }

    pub fn distance_to_line(&self, line: &Line) -> f32 {
        self.distance_to_line_squared(line).sqrt()
    }

    pub fn distance_to_line_squared(&self, line: &Line) -> f32 {
        let self_diff = self.as_difference();
        let line_diff = line.as_difference();
        let to_start = self.start - line.start;

        let a = self_diff.dot(self_diff); // always >= 0
        let b = self_diff.dot(line_diff);
        let c = line_diff.dot(line_diff); // always >= 0
        let d = self_diff.dot(to_start);
        let e = line_diff.dot(to_start);
        let D = (a * c) - (b * b); // always >= 0

        let mut s_n; // sc = sN / sD, default sD = D >= 0
        let mut s_d = D;
        let mut t_n; // tc = tN / tD, default tD = D >= 0
        let mut t_d = D;

        let small_num = 0.000000001;

        // compute the line parameters of the two closest points
        if D < small_num {
            // the lines are almost parallel
            s_n = 0.0; // force using point P0 on segment S1
            s_d = 1.0; // to prevent possible division by 0.0 later
            t_n = e;
            t_d = c;
        } else {
            // get the closest points on the infinite lines
            s_n = (b * e) - (c * d);
            t_n = (a * e) - (b * d);

            if s_n < 0.0 {
                // sc < 0 => the s=0 edge is visible
                s_n = 0.0;
                t_n = e;
                t_d = c;
            } else if s_n > s_d {
                // sc > 1  => the s=1 edge is visible
                s_n = s_d;
                t_n = e + b;
                t_d = c;
            }
        }

        if t_n < 0.0 {
            // tc < 0 => the t=0 edge is visible
            t_n = 0.0;

            // recompute sc for this edge
            if -d < 0.0 {
                s_n = 0.0;
            } else if -d > a {
                s_n = s_d;
            } else {
                s_n = -d;
                s_d = a;
            }
        } else if t_n > t_d {
            // tc > 1  => the t=1 edge is visible
            t_n = t_d;

            // recompute sc for this edge
            if (-d + b) < 0.0 {
                s_n = 0.0;
            } else if (-d + b) > a {
                s_n = s_d;
            } else {
                s_n = -d + b;
                s_d = a;
            }
        }

        // finally do the division to get sc and tc
        let sc = if s_n.abs() < small_num {
            0.0
        } else {
            s_n / s_d
        };

        let tc = if t_n.abs() < small_num {
            0.0
        } else {
            t_n / t_d
        };

        // get the difference of the two closest points
        let d_p = to_start + (sc * self_diff) - (tc * line_diff); // =  S1(sc) - S2(tc)

        d_p.length_squared() // return the closest distance
    }
}

impl From<(Vec2, Vec2)> for Line {
    fn from((start, end): (Vec2, Vec2)) -> Self {
        Self { start, end }
    }
}

impl From<[Vec2; 2]> for Line {
    fn from([start, end]: [Vec2; 2]) -> Self {
        Self { start, end }
    }
}

impl Transformable for Line {
    fn to_transformed(&self, transform: &GlobalTransform) -> Self {
        Self::new(
            transform.transform_point2(self.start),
            transform.transform_point2(self.end),
        )
    }
}

impl Collides<Capsule> for Line {
    fn collide(&self, other: &Capsule) -> CollisionResult {
        other.collide(self)
    }
}

impl Collides<Circle> for Line {
    fn collide(&self, other: &Circle) -> CollisionResult {
        other.collide(self)
    }
}

impl Collides<Line> for Line {
    fn collide(&self, other: &Line) -> CollisionResult {
        (self.distance_to_line_squared(other) <= f32::EPSILON).into()
    }
}

impl Collides<Point> for Line {
    fn collide(&self, other: &Point) -> CollisionResult {
        (self.distance_to_point_squared(&other.0) <= f32::EPSILON).into()
    }
}

impl Collides<Polygon> for Line {
    fn collide(&self, other: &Polygon) -> CollisionResult {
        other.collide(self)
    }
}

impl Collides<Rectangle> for Line {
    fn collide(&self, other: &Rectangle) -> CollisionResult {
        (other.contains(self.start)
            || other.contains(self.end)
            || self.collide(&other.left()).colliding
            || self.collide(&other.top()).colliding
            || self.collide(&other.right()).colliding
            || self.collide(&other.bottom()).colliding)
            .into()
    }
}

impl Collides<Triangle> for Line {
    fn collide(&self, other: &Triangle) -> CollisionResult {
        todo!()
    }
}

#[cfg(feature = "debug-draw")]
impl Geometry for Line {
    fn add_geometry(&self, b: &mut Builder) {
        b.add_line_segment(&LineSegment {
            from: (self.start.x, self.start.y).into(),
            to: (self.end.x, self.end.y).into(),
        });
    }
}
