use super::*;

pub fn collide_capsule_capsule(us: &Capsule, them: &Capsule) -> ContactManifold {
    let closest_points = us.line.closest_point_to_line(&them.line);
    if closest_points.is_point() {
        // The inner lines of the capsules are intersecting
        ContactManifold::coincident(closest_points.start)
    } else {
        let diff = closest_points.as_difference();
        if diff.length_squared() <= (us.radius + them.radius).powi(2) {
            if us.line.is_parallel_to(&them.line) {
                if let Some(us_clipped) = us.line.clip_to_parallel_line(&them.line) {
                    if let Some(them_clipped) = them.line.clip_to_parallel_line(&us.line) {
                        // The capsules are parallel
                        return ContactManifold::edge(us_clipped, them_clipped, diff.normalize());
                    }
                }
            }

            // Non-parallel capsules can be treated like a pair of circles around the closest points
            ContactManifold::point(
                closest_points.start + diff.clamp_length(us.radius, us.radius),
                closest_points.end - diff.clamp_length(them.radius, them.radius),
                diff.normalize(),
            )
        } else {
            ContactManifold::disjoint()
        }
    }
}

pub fn collide_capsule_circle(us: &Capsule, them: &Circle) -> ContactManifold {
    let closest_point = us.line.closest_point_to_point(&them.position);
    let diff = them.position - closest_point;
    ContactManifold::new_lazy(
        diff.length_squared() <= (us.radius + them.radius).powi(2),
        || {
            ContactPoint::new(
                closest_point + diff.clamp_length(us.radius, us.radius),
                them.position - diff.clamp_length(them.radius, them.radius),
                diff.normalize(),
            )
            .into()
        },
    )
}

// pub fn collide_capsule_line(us: &Capsule, them: &Line) -> ContactManifold {
//     let closest_points = us.line.closest_point_to_line(them);
//     if closest_points.length_squared() <= us.radius_squared() {
//         if closest_points.is_point() {
//             ContactManifold::Coincident(closest_points.start).into()
//         } else if us.line.is_parallel_to(them) {
//             ContactEdge::new().into()
//         } else {
//             ContactPoint::new(
//                 closest_points.start + closest_points.as_difference().clamp_length_min(us.radius),
//                 closest_points.end,
//                 closest_points.as_difference().normalize(),
//             )
//             .into()
//         }
//     } else {
//         ContactManifold::Disjoint
//     }
// }

pub fn collide_capsule_point(us: &Capsule, them: &Point) -> ContactManifold {
    if us.line.distance_to_point_squared(&them.0) <= us.radius_squared() {
        ContactManifold::Coincident(them.0)
    } else {
        ContactManifold::Disjoint
    }
}

pub fn collide_capsule_polygon(us: &Capsule, them: &Polygon) -> ContactManifold {}

pub fn collide_capsule_rect(us: &Capsule, them: &Rectangle) -> ContactManifold {}

pub fn collide_circle_circle(us: &Circle, them: &Circle) -> ContactManifold {
    let diff = them.position - us.position;
    ContactManifold::new_lazy(
        diff.length_squared() <= (us.radius + them.radius).powi(2),
        || {
            ContactPoint::new(
                us.position + diff.clamp_length(us.radius, us.radius),
                them.position - diff.clamp_length(them.radius, them.radius),
                diff.normalize(),
            )
            .into()
        },
    )
}

pub fn collide_circle_line(us: &Circle, them: &Line) -> ContactManifold {}

pub fn collide_circle_point(us: &Circle, them: &Point) -> ContactManifold {
    ContactManifold::new_lazy(
        us.position.distance_squared(them.0) <= us.radius_squared(),
        || them.0.into(),
    )
}

pub fn collide_circle_polygon(us: &Circle, them: &Polygon) -> ContactManifold {}

pub fn collide_circle_rect(us: &Circle, them: &Rectangle) -> ContactManifold {
    let (closest_point, center_is_in_rect) = them.closest_point_on_perimeter(us.position);
    if center_is_in_rect {
        let normal = (us.position - closest_point).normalize();
        ContactManifold::point(us.support_point(normal), closest_point, normal)
    } else if us.contains(closest_point) {
        let normal = (closest_point - us.position).normalize();
        ContactManifold::point(us.support_point(normal), closest_point, normal)
    } else {
        ContactManifold::disjoint()
    }
}

pub fn collide_line_line(us: &Line, them: &Line) -> ContactManifold {
    match us.intersect_line(them) {
        LineIntersection::Disjoint => ContactManifold::Disjoint,
        LineIntersection::Intersecting(point) => point.into(),
        LineIntersection::Colinear(line) => line.parametric_point(0.5).into(),
    }
}

pub fn collide_line_point(us: &Line, them: &Point) -> ContactManifold {
    ContactManifold::new_lazy(
        us.distance_to_point_squared(&them.0) <= f32::EPSILON * f32::EPSILON,
        || them.0.into(),
    )
}

pub fn collide_line_polygon(us: &Line, them: &Polygon) -> ContactManifold {}

pub fn collide_line_rect(us: &Line, them: &Rectangle) -> ContactManifold {}

pub fn collide_point_point(us: &Point, them: &Point) -> ContactManifold {
    ContactManifold::new_lazy(
        us.distance_squared(them.0) <= f32::EPSILON * f32::EPSILON,
        || us.0.into(),
    )
}

pub fn collide_point_polygon(us: &Point, them: &Polygon) -> ContactManifold {}

pub fn collide_point_rect(us: &Point, them: &Rectangle) -> ContactManifold {
    ContactManifold::new_lazy(them.contains(us.0), || us.0.into())
}

pub fn collide_polygon_polygon(us: &Polygon, them: &Polygon) -> ContactManifold {}

pub fn collide_polygon_rect(us: &Polygon, them: &Rectangle) -> ContactManifold {}

pub fn collide_rect_rect(us: &Rectangle, them: &Rectangle) -> ContactManifold {
    enum RectAxis {
        Left,
        Top,
        Right,
        Bottom,
    }

    // The penetration depths of each side of the rects
    let left = them.max().x - us.min().x;
    let top = us.max().y - them.min().y;
    let right = us.max().x - them.min().x;
    let bottom = them.max().y - us.min().y;

    // Negative penetration depths represent a valid separating axis, meaning the rects are not colliding, so
    // all of the penetration depths must be at least 0 (A penetration depth of exactly 0 counts as colliding)
    let minimum_penetrating_axis = if left <= top && left <= right && left <= bottom {
        (0.0 <= left).then_some(RectAxis::Left)
    } else if top <= right && top <= bottom {
        (0.0 <= top).then_some(RectAxis::Top)
    } else if right <= bottom {
        (0.0 <= right).then_some(RectAxis::Right)
    } else {
        (0.0 <= bottom).then_some(RectAxis::Bottom)
    };

    minimum_penetrating_axis
        .map(|axis| {
            use RectAxis::*;
            let intersection = Rectangle::new(us.0.intersect(them.0));
            match axis {
                Left => ContactEdge::new(intersection.left(), intersection.right(), Vec2::NEG_X),
                Top => ContactEdge::new(intersection.top(), intersection.bottom(), Vec2::Y),
                Right => ContactEdge::new(intersection.right(), intersection.left(), Vec2::X),
                Bottom => ContactEdge::new(intersection.bottom(), intersection.top(), Vec2::NEG_Y),
            }
        })
        .into()
}
