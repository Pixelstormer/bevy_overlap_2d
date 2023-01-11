use super::*;
use std::ops::Neg;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ContactManifold {
    Disjoint,
    Point(ContactPoint),
    Edge(ContactEdge),
    Coincident(Vec2),
}

impl ContactManifold {
    pub fn new_lazy(colliding: bool, contact: impl FnOnce() -> Self) -> Self {
        if colliding {
            contact()
        } else {
            Self::Disjoint
        }
    }

    pub fn negate(&mut self) {
        *self = self.neg();
    }

    pub fn disjoint(&self) -> bool {
        matches!(self, Self::Disjoint)
    }

    pub fn colliding(&self) -> bool {
        !self.disjoint()
    }
}

impl Neg for ContactManifold {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        match self {
            Self::Disjoint => Self::Disjoint,
            Self::Point(point) => Self::Point(point.neg()),
            Self::Edge(edge) => Self::Edge(edge.neg()),
            Self::Coincident(point) => Self::Coincident(point),
        }
    }
}

impl From<ContactPoint> for ContactManifold {
    fn from(point: ContactPoint) -> Self {
        Self::Point(point)
    }
}

impl From<ContactEdge> for ContactManifold {
    fn from(edge: ContactEdge) -> Self {
        Self::Edge(edge)
    }
}

impl From<Vec2> for ContactManifold {
    fn from(point: Vec2) -> Self {
        Self::Coincident(point)
    }
}

impl<T: Into<ContactManifold>> From<Option<T>> for ContactManifold {
    fn from(contact: Option<T>) -> Self {
        contact.map_or(Self::Disjoint, Into::into)
    }
}

#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct ContactPoint {
    us: Vec2,
    them: Vec2,
    normal: Vec2,
}

impl ContactPoint {
    pub fn new(us: Vec2, them: Vec2, normal: Vec2) -> Self {
        Self { us, them, normal }
    }

    pub fn separation_vector(&self) -> Vec2 {
        self.them - self.us
    }

    pub fn penetration_depth(&self) -> f32 {
        self.separation_vector().dot(self.normal)
    }

    pub fn negate(&mut self) {
        *self = self.neg();
    }
}

impl Neg for ContactPoint {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            us: self.them,
            them: self.us,
            normal: -self.normal,
        }
    }
}

#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct ContactEdge {
    us: Line,
    them: Line,
    normal: Vec2,
}

impl ContactEdge {
    pub fn new(us: Line, them: Line, normal: Vec2) -> Self {
        Self { us, them, normal }
    }

    pub fn negate(&mut self) {
        *self = self.neg();
    }
}

impl Neg for ContactEdge {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            us: self.them,
            them: self.us,
            normal: -self.normal,
        }
    }
}
