use crate::ray::{HitRecord, Ray};
use crate::{ray::Hittable, vec3::Point};

struct Sphere {
    center: Point,
    radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64, record: &mut HitRecord) -> bool {
        let oc = &self.center - ray.origin();
        let a = ray.direction().norm_square();
        let h = ray.direction().dot(&oc);
        let c = &oc.norm_square() - self.radius.powi(2);
        let discriminant = h.powi(2) - a * c;

        if discriminant < 0. {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let root = (h - sqrtd) / a;

        if root <= ray_tmin || ray_tmax <= root {
            let root = (h + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return false;
            }
        }

        record.t = root;
        record.p = ray.at(record.t);
        record.normal = &(&record.p - &self.center) / self.radius;

        return true;
    }
}
