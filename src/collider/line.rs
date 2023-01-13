use super::*;
use crate::transform_ext::TransformPoint2;
use bevy_prototype_lyon::prelude::tess::geom::LineSegment;

/// A straight line connecting two points.
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Line {
    pub start: Vec2,
    pub end: Vec2,
}

#[derive(Clone, Copy, Debug)]
pub enum LineIntersection {
    Disjoint,
    Intersecting(Vec2),
    Colinear(Line),
}

impl Line {
    pub fn new(start: Vec2, end: Vec2) -> Self {
        Self { start, end }
    }

    pub fn length(&self) -> f32 {
        self.as_difference().length()
    }

    pub fn length_squared(&self) -> f32 {
        self.as_difference().length_squared()
    }

    pub fn as_difference(&self) -> Vec2 {
        self.end - self.start
    }

    pub fn perp(&self) -> Vec2 {
        self.as_difference().perp()
    }

    pub fn is_point(&self) -> bool {
        self.start == self.end
    }

    pub fn is_parallel_to(&self, other: &Self) -> bool {
        self.as_difference().perp_dot(other.as_difference()).abs() <= f32::EPSILON
    }

    pub fn is_vertical(&self) -> bool {
        (self.start.x - self.end.x).abs() <= f32::EPSILON
    }

    pub fn is_horizontal(&self) -> bool {
        (self.start.y - self.end.y).abs() <= f32::EPSILON
    }

    /// Gets a point on the infinite line through `self` according to the parametric line equation:
    ///
    /// Negative values of `t` return points 'before' `self.start`.
    /// `t = 0` returns `self.start`.
    /// Values of `t` between 0 and 1 return points on `self`.
    /// `t = 1` returns `self.end`.
    /// Values of `t` greater than 1 return points 'after' `self.end`.
    pub fn parametric_point(&self, t: f32) -> Vec2 {
        self.start + (self.as_difference() * t)
    }

    /// The y coordinate of the infinite line through `self` at the given x coordinate
    ///
    /// When `self` is a vertical line, the return value is arbitrary and meaningless
    pub fn y_at_x(&self, x: f32) -> f32 {
        let perp_dot = self.end.perp_dot(self.start);
        let diff = self.as_difference();
        (perp_dot + (x * (diff.y))) / (diff.x)
    }

    /// The x coordinate of the infinite line through `self` at the given y coordinate
    ///
    /// When `self` is a horizontal line, the return value is arbitrary and meaningless
    pub fn x_at_y(&self, y: f32) -> f32 {
        ((self.end.x * (self.start.y - y)) + (self.start.x * (y - self.end.y)))
            / (self.start.y - self.end.y)
    }

    /// Checks whether or not `point` intersects `self`.
    ///
    /// `point` must lie somewhere on the infinite line through `self`.
    pub fn intersects_colinear_point(&self, point: &Vec2) -> bool {
        if self.start.x != self.end.x {
            // S is not vertical
            (self.start.x <= point.x && point.x <= self.end.x)
                || (self.start.x >= point.x && point.x >= self.end.x)
        } else {
            // S is vertical, so test y coordinate
            (self.start.y <= point.y && point.y <= self.end.y)
                || (self.start.y >= point.y && point.y >= self.end.y)
        }
    }

    /// Clips `self` to `other`. Returns `None` if there is no overlap between `self` and `other`.
    ///
    /// `self` and `other` must be parallel.
    pub fn clip_to_parallel_line(&self, other: &Self) -> Option<Self> {
        debug_assert!(self.is_parallel_to(other));

        let direction = self.as_difference().normalize();

        let start_pos = direction.dot(other.start);
        let end_pos = direction.dot(other.end);
        if start_pos > end_pos {
            std::mem::swap(&mut start_pos, &mut end_pos);
        }

        let new_start = if start_pos <= 0.0 {
            if end_pos < 0.0 {
                return None;
            }

            self.start
        } else {
            self.start + (direction * start_pos)
        };

        let self_end_pos = direction.dot(self.end);
        let new_end = if end_pos >= self_end_pos {
            if start_pos > self_end_pos {
                return None;
            }

            self.end
        } else {
            self.start + (direction * end_pos)
        };

        Some(Self::new(new_start, new_end))
    }

    /// Projects `self` and `other` on one another and compute their intersection.
    ///
    /// Taken from https://github.com/dimforge/parry/blob/f7db00739e4fab0dc92e51d56b643e246339cf65/src/query/clip/clip_segment_segment.rs#L76
    pub fn clip_with(&self, other: &Self) -> Option<(Self, Self)> {
        // NOTE: no need to normalize the tangent.
        let tangent1 = self.as_difference();

        let mut range1 = [0.0, tangent1.length_squared()];
        let mut range2 = [
            (other.start - self.start).dot(tangent1),
            (other.end - self.start).dot(tangent1),
        ];

        if range1[1] < range1[0] {
            range1.swap(0, 1);
            std::mem::swap(&mut self.start, &mut self.end);
        }

        if range2[1] < range2[0] {
            range2.swap(0, 1);
            std::mem::swap(&mut other.start, &mut other.end);
        }

        if range2[0] > range1[1] || range1[0] > range2[1] {
            // No clip point.
            return None;
        }

        let length1 = range1[1] - range1[0];
        let length2 = range2[1] - range2[0];

        let ca = if range2[0] > range1[0] {
            let bcoord = (range2[0] - range1[0]) / length1;
            Self::new(self.start + (tangent1 * bcoord), other.start)
        } else {
            let bcoord = (range1[0] - range2[0]) / length2;
            Self::new(
                self.start,
                other.start + ((other.end - other.start) * bcoord),
            )
        };

        let cb = if range2[1] < range1[1] {
            let bcoord = (range2[1] - range1[0]) / length1;
            Self::new(self.start + (tangent1 * bcoord), other.end)
        } else {
            let bcoord = (range1[1] - range2[0]) / length2;
            Self::new(self.end, other.start + ((other.end - other.start) * bcoord))
        };

        Some((ca, cb))
    }

    /// Computes the intersection between `self` and `other`, if any exists.
    ///
    /// From https://web.archive.org/web/20210428000731/http://geomalgorithms.com/a05-_intersect-1.html
    pub fn intersect_line(&self, other: &Self) -> LineIntersection {
        let u = self.as_difference();
        let v = other.as_difference();
        let w = self.start - other.start;
        let d = u.perp_dot(v);

        // get the intersect parameter for S1
        let si = v.perp_dot(w) / d;

        // test if they are parallel (includes either being a point)
        if si.is_finite() {
            // the segments are skew and may intersect in a point
            if si < 0.0 || si > 1.0 {
                // no intersect with S1
                LineIntersection::Disjoint
            } else {
                // get the intersect parameter for S2
                let ti = u.perp_dot(w) / d;
                if ti < 0.0 || ti > 1.0 {
                    // no intersect with S2
                    LineIntersection::Disjoint
                } else {
                    LineIntersection::Intersecting(self.parametric_point(si))
                }
            }
        } else {
            // S1 and S2 are parallel
            if u.perp_dot(w) != 0.0 || v.perp_dot(w) != 0.0 {
                LineIntersection::Disjoint // they are NOT collinear
            } else {
                // they are collinear or degenerate
                // check if they are degenerate points
                match (self.is_point(), other.is_point()) {
                    // both segments are points
                    (true, true) => (self == other)
                        .then_some(LineIntersection::Intersecting(self.start))
                        .unwrap_or(LineIntersection::Disjoint),
                    // S1 is a single point
                    (true, false) => other
                        .intersects_colinear_point(&self.start)
                        .then_some(LineIntersection::Intersecting(self.start))
                        .unwrap_or(LineIntersection::Disjoint),
                    // S2 a single point
                    (false, true) => self
                        .intersects_colinear_point(&other.start)
                        .then_some(LineIntersection::Intersecting(other.start))
                        .unwrap_or(LineIntersection::Disjoint),
                    (false, false) => {
                        // they are collinear segments - get overlap (or not)

                        // endpoints of S1 in eqn for S2
                        let t0;
                        let t1;
                        let w2 = self.end - other.start;

                        if v.x != 0.0 {
                            t0 = w.x / v.x;
                            t1 = w2.x / v.x;
                        } else {
                            t0 = w.y / v.y;
                            t1 = w2.y / v.y;
                        }

                        if t0 > t1 {
                            // must have t0 smaller than t1
                            let t = t0;
                            t0 = t1;
                            t1 = t; // swap if not
                        }

                        if t0 > 1.0 || t1 < 0.0 {
                            LineIntersection::Disjoint // NO overlap
                        } else {
                            t0 = t0.max(0.0); // clip to min 0
                            t1 = t1.min(1.0); // clip to max 1

                            if t0 == t1 {
                                // intersect is a point
                                LineIntersection::Intersecting(other.parametric_point(t0))
                            } else {
                                // they overlap in a valid subsegment
                                LineIntersection::Colinear(Self::new(
                                    other.parametric_point(t0),
                                    other.parametric_point(t1),
                                ))
                            }
                        }
                    }
                }
            }
        }
    }

    /// The closest point on `self` to the given point.
    ///
    /// From https://web.archive.org/web/20210507021429/http://geomalgorithms.com/a02-_lines.html
    pub fn closest_point_to_point(&self, point: &Vec2) -> Vec2 {
        let v = self.as_difference();
        let w = *point - self.start;

        let c1 = w.dot(v);
        if c1 <= 0.0 {
            return self.start;
        }

        let c2 = v.length_squared();
        if c2 <= c1 {
            return self.end;
        }

        let b = c1 / c2;
        let pb = self.start + (b * v);
        return pb;
    }

    /// The smallest distance between `self` and the given point.
    pub fn distance_to_point(&self, point: &Vec2) -> f32 {
        self.distance_to_point_squared(point).sqrt()
    }

    /// The square of the smallest distance between `self` and the given point.
    pub fn distance_to_point_squared(&self, point: &Vec2) -> f32 {
        self.closest_point_to_point(point).distance_squared(*point)
    }

    /// A line between the closest points on `self` and `other`.
    ///
    /// From https://web.archive.org/web/20210410192029/http://geomalgorithms.com/a07-_distance.html
    pub fn closest_point_to_line(&self, other: &Self) -> Self {
        let u = self.as_difference();
        let v = other.as_difference();
        let w = self.start - other.start;

        let a = u.length_squared(); // always >= 0
        let b = u.dot(v);
        let c = v.length_squared(); // always >= 0
        let d = u.dot(w);
        let e = v.dot(w);

        let denominator = (a * c) - (b * b); // always >= 0
        let mut sn; // sc = sN / sD, default sD = D >= 0
        let mut sd = denominator;
        let mut tn; // tc = tN / tD, default tD = D >= 0
        let mut td = denominator;

        // compute the line parameters of the two closest points
        if denominator <= f32::EPSILON {
            // the lines are almost parallel
            sn = 0.0; // force using point P0 on segment S1
            sd = 1.0; // to prevent possible division by 0.0 later
            tn = e;
            td = c;
        } else {
            // get the closest points on the infinite lines
            sn = (b * e) - (c * d);
            tn = (a * e) - (b * d);

            if sn < 0.0 {
                // sc < 0 => the s=0 edge is visible
                sn = 0.0;
                tn = e;
                td = c;
            } else if sn > sd {
                // sc > 1  => the s=1 edge is visible
                sn = sd;
                tn = e + b;
                td = c;
            }
        }

        if tn < 0.0 {
            // tc < 0 => the t=0 edge is visible
            tn = 0.0;

            // recompute sc for this edge
            if -d < 0.0 {
                sn = 0.0;
            } else if -d > a {
                sn = sd;
            } else {
                sn = -d;
                sd = a;
            }
        } else if tn > td {
            // tc > 1  => the t=1 edge is visible
            tn = td;

            // recompute sc for this edge
            if (-d + b) < 0.0 {
                sn = 0.0;
            } else if (-d + b) > a {
                sn = sd;
            } else {
                sn = -d + b;
                sd = a;
            }
        }

        // finally do the division to get sc and tc
        let sc = if sn.abs() <= f32::EPSILON {
            0.0
        } else {
            sn / sd
        };

        let tc = if tn.abs() <= f32::EPSILON {
            0.0
        } else {
            tn / td
        };

        Self::new(self.parametric_point(sc), other.parametric_point(tc))
    }

    pub fn distance_to_line_squared(&self, other: &Self) -> f32 {
        self.closest_point_to_line(other).length_squared()
    }

    pub fn distance_to_line(&self, other: &Self) -> f32 {
        self.distance_to_line_squared(other).sqrt()
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
    fn collide(&self, other: &Capsule) -> ContactManifold {
        other.collide(self)
    }
}

impl Collides<Circle> for Line {
    fn collide(&self, other: &Circle) -> ContactManifold {
        other.collide(self)
    }
}

impl Collides<Line> for Line {
    fn collide(&self, other: &Self) -> ContactManifold {
        algorithms::collide_line_line(self, other)
    }
}

impl Collides<Point> for Line {
    fn collide(&self, other: &Point) -> ContactManifold {
        (self.distance_to_point_squared(&other.0) <= f32::EPSILON * f32::EPSILON).into()
    }
}

impl Collides<Polygon> for Line {
    fn collide(&self, other: &Polygon) -> ContactManifold {
        other.collide(self)
    }
}

impl Collides<Rectangle> for Line {
    fn collide(&self, other: &Rectangle) -> ContactManifold {
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
    fn collide(&self, other: &Triangle) -> ContactManifold {
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
