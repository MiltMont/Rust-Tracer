use crate::{
    color::Color,
    vec3::{Point, Vec3},
};

pub struct Ray {
    origin: Point,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> &Point {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn at(&self, t: f64) -> Point {
        return &self.origin + &(t * (&self.direction));
    }
}

// Temporal
pub fn hit_sphere(center: &Point, radius: f64, ray: &Ray) -> f64 {
    let oc = center - ray.origin();
    let a = ray.direction().norm_square();
    let h = ray.direction().dot(&oc);
    let c = &oc.norm_square() - radius.powi(2);
    let discriminant = h.powi(2) - a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (h - discriminant.sqrt()) / a;
    }
}

pub fn ray_color(ray: &Ray) -> Color {
    let t = hit_sphere(&Point::new(0.0, 0.0, -1.0), 0.5, ray);

    if t > 0.0 {
        let normal = (ray.at(t) - Vec3::new(0.0, 0.0, -1.0)).normalize();
        return 0.5 * Color::new(normal.x() + 1., normal.y() + 1., normal.z() + 1.);
    }

    let unit_direction = ray.direction().normalize();
    let a = 0.5 * (unit_direction.y() + 1.0);

    // Going from White to Blue
    return (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
}

pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64, record: &mut HitRecord) -> bool;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_at() {
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let mid = ray.at(0.5);

        k9::snapshot!(mid, "(0, 0, 0.5)");
    }
}
