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

pub fn ray_color(r: &Ray) -> Color {
    let unit_direction = r.direction().normalize();
    let a = 0.5 * (unit_direction.y() + 1.0);

    // Going from White to Blue
    let color = (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
    color
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
