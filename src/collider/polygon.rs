use super::*;
use crate::transform_ext::TransformPoint2;
use std::iter::FusedIterator;

/// An arbitrary polygon.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Polygon {
    pub vertices: Vec<Vec2>,
}

impl Polygon {
    pub fn new(vertices: Vec<Vec2>) -> Self {
        Self { vertices }
    }

    pub fn edges(&self) -> EdgesIterator<'_> {
        EdgesIterator::new(&self.vertices)
    }

    pub fn crossing_number(&self, point: Vec2) -> u32 {
        let mut crossing_number = 0;

        for edge in self.edges() {
            // An upward crossing
            if ((edge.start.y <= point.y) && (edge.end.y > point.y))
            // A downward crossing
            || ((edge.start.y > point.y) && (edge.end.y <= point.y))
            {
                // Compute the actual edge-ray intersect x-coordinate
                let intersect = (point.y - edge.start.y) / (edge.end.y - edge.start.y);
                if point.x < edge.start.x + (intersect * (edge.end.x - edge.start.x)) {
                    crossing_number += 1; // A valid crossing of y=point.y right of point.x
                }
            }
        }

        crossing_number // & 1 // 0 if even (out), and 1 if odd (in)
    }

    pub fn winding_number(&self, point: Vec2) -> i32 {
        let mut winding_number = 0;

        for edge in self.edges() {
            if edge.start.y <= point.y {
                // Start y <= point.y
                if edge.end.y > point.y {
                    // An upward crossing
                    if ((edge.end.x - edge.start.x) * (point.y - edge.start.y))
                        - ((point.x - edge.start.x) * (edge.end.y - edge.start.y))
                        > 0.0
                    {
                        // point left of edge
                        winding_number += 1; // Have a valid up intersect
                    }
                }
            } else {
                // Start y > point.y (no test needed)
                if edge.end.y <= point.y {
                    // A downward crossing
                    if ((edge.end.x - edge.start.x) * (point.y - edge.start.y))
                        - ((point.x - edge.start.x) * (edge.end.y - edge.start.y))
                        < 0.0
                    {
                        // point right of edge
                        winding_number -= 1; // Have a valid down intersect
                    }
                }
            }
        }

        winding_number
    }

    pub fn contains(&self, point: Vec2) -> bool {
        self.winding_number(point) != 0
    }
}

impl From<Vec<Vec2>> for Polygon {
    fn from(vertices: Vec<Vec2>) -> Self {
        Self { vertices }
    }
}

impl Transformable for Polygon {
    fn to_transformed(&self, transform: &GlobalTransform) -> Self {
        Self::new(
            self.vertices
                .iter()
                .map(|&vertex| transform.transform_point2(vertex))
                .collect(),
        )
    }
}

impl Collides<Capsule> for Polygon {
    fn collide(&self, other: &Capsule) -> Option<ContactManifold> {
        (self.contains(other.line.start)
            || self.contains(other.line.end)
            || self.edges().any(|edge| edge.collide(other).colliding))
        .into()
    }
}

impl Collides<Circle> for Polygon {
    fn collide(&self, other: &Circle) -> Option<ContactManifold> {
        (self.contains(other.position) || self.edges().any(|edge| edge.collide(other).colliding))
            .into()
    }
}

impl Collides<Line> for Polygon {
    fn collide(&self, other: &Line) -> Option<ContactManifold> {
        (self.contains(other.start)
            || self.contains(other.end)
            || self.edges().any(|edge| edge.collide(other).colliding))
        .into()
    }
}

impl Collides<Point> for Polygon {
    fn collide(&self, other: &Point) -> Option<ContactManifold> {
        self.contains(other.0).into()
    }
}

impl Collides<Polygon> for Polygon {
    fn collide(&self, other: &Polygon) -> Option<ContactManifold> {
        (self.vertices.iter().any(|&vertex| other.contains(vertex))
            || other.vertices.iter().any(|&vertex| self.contains(vertex))
            || self.edges().any(|edge| edge.collide(other).colliding))
        .into()
    }
}

impl Collides<Rectangle> for Polygon {
    fn collide(&self, other: &Rectangle) -> Option<ContactManifold> {
        (self.contains(other.min())
            || self.contains(other.max())
            || self.edges().any(|edge| edge.collide(other).colliding))
        .into()
    }
}

impl Collides<Triangle> for Polygon {
    fn collide(&self, other: &Triangle) -> Option<ContactManifold> {
        todo!()
    }
}

#[cfg(feature = "debug-draw")]
impl Geometry for Polygon {
    fn add_geometry(&self, b: &mut Builder) {
        let mut iter = self.vertices.iter();

        let Some(first) = iter.next() else { return; };

        b.begin((first.x, first.y).into());

        for vertex in iter {
            b.line_to((vertex.x, vertex.y).into());
        }

        b.end(true);
    }
}

#[derive(Clone, Debug)]
pub struct EdgesIterator<'a> {
    vertices: &'a [Vec2],
    first: Option<Vec2>,
}

impl<'a> EdgesIterator<'a> {
    pub fn new(vertices: &'a [Vec2]) -> Self {
        Self {
            vertices,
            first: None,
        }
    }
}

impl Iterator for EdgesIterator<'_> {
    type Item = Line;

    fn next(&mut self) -> Option<Self::Item> {
        match self.vertices {
            [] => None,
            &[start] => self.first.take().map(|end| Line::new(start, end)),
            &[start, end, ..] => {
                self.first.get_or_insert(start);
                self.vertices = &self.vertices[1..];
                Some(Line::new(start, end))
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.vertices.len();
        if len == 1 && self.first.is_none() {
            (0, Some(0))
        } else {
            (len, Some(len))
        }
    }

    fn count(self) -> usize {
        self.len()
    }
}

impl ExactSizeIterator for EdgesIterator<'_> {}

impl FusedIterator for EdgesIterator<'_> {}
