use bevy::{
    math::Affine3A,
    prelude::{GlobalTransform, Vec2},
};

pub trait TransformPoint2 {
    fn transform_point2(&self, point: Vec2) -> Vec2;
    fn transform_vec2(&self, vec: Vec2) -> Vec2;
}

impl TransformPoint2 for GlobalTransform {
    fn transform_point2(&self, point: Vec2) -> Vec2 {
        self.affine().transform_point2(point)
    }

    fn transform_vec2(&self, vec: Vec2) -> Vec2 {
        self.affine().transform_vec2(vec)
    }
}

impl TransformPoint2 for Affine3A {
    fn transform_point2(&self, point: Vec2) -> Vec2 {
        ((self.matrix3.x_axis * point.x) + (self.matrix3.y_axis * point.y) + self.translation)
            .truncate()
    }

    fn transform_vec2(&self, vec: Vec2) -> Vec2 {
        ((self.matrix3.x_axis * vec.x) + (self.matrix3.y_axis * vec.y)).truncate()
    }
}
